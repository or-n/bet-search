pub mod download;
pub mod page;
// pub mod probability;

pub fn split2(x: String, on: &str) -> Option<[String; 2]> {
    let parts: Vec<_> = x.split(on).collect();
    if parts.len() != 2 {
        return None;
    }
    let a = parts[0].to_string();
    let b = parts[1].to_string();
    Some([a, b])
}

pub const ENTER: &str = "\u{E007}";
pub const _TAB: &str = "\u{E004}";
pub const _ESC: &str = "\u{E00C}";

pub fn localhost(port: u16) -> String {
    format!("http://localhost:{}", port)
}
