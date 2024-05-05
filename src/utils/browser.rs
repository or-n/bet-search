use fantoccini::ClientBuilder;

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
