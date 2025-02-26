use std::process::{Child, Command};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Pool<T> {
    ids: Arc<Mutex<Ids<T>>>,
    spawned: Vec<Child>,
}

struct Ids<T> {
    free: Vec<T>,
    taken: Vec<T>,
}

impl<T: PartialEq + Copy> Pool<T> {
    pub fn new(ids: Vec<T>) -> Self {
        Self {
            ids: Arc::new(Mutex::new(Ids {
                free: ids,
                taken: Vec::new(),
            })),
            spawned: Vec::new(),
        }
    }

    // fn spawn(id: u16) -> std::io::Result<Child> {
    //     Command::new("geckodriver")
    //         .arg("--port")
    //         .arg(port.to_string())
    //         .spawn()
    // }

    // pub async fn initialize(&mut self) -> Result<(), std::io::Error> {
    //     let ports = self.ports.lock().await;
    //     for &port in &ports.free {
    //         let child = Self::spawn(port)?;
    //         self.spawned.push(child);
    //     }
    //     Ok(())
    // }

    pub async fn acquire(&self) -> Option<T> {
        let mut ids = self.ids.lock().await;
        let id = ids.free.pop()?;
        ids.taken.push(id);
        Some(id)
    }

    pub async fn release(&self, id: T) {
        let mut ids = self.ids.lock().await;
        if let Some(index) = ids.taken.iter().position(|&x| x == id) {
            ids.taken.remove(index);
            ids.free.push(id);
        }
    }
}

impl<T> Drop for Pool<T> {
    fn drop(&mut self) {
        for child in self.spawned.iter_mut() {
            let _ = child.kill();
        }
    }
}
