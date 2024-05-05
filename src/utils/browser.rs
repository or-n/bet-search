use fantoccini::ClientBuilder;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Error {
    port: u16,
    error: fantoccini::error::NewSessionError,
}

pub async fn client(port: u16) -> Result<fantoccini::Client, Error> {
    ClientBuilder::native()
        .connect(format!("http://localhost:{}", port).as_str())
        .await
        .map_err(|error| Error { port, error })
}

pub struct Browser<T> {
    pub port: u16,
    _marker: PhantomData<T>,
}

impl<T> Browser<T> {
    pub fn new(port: u16) -> Self {
        Browser {
            port,
            _marker: PhantomData,
        }
    }
}
