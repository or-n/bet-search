#![allow(async_fn_in_trait)]
pub trait Download<Client, Data>: Sized {
    type Error;

    async fn download(client: &Client, data: Data)
        -> Result<Self, Self::Error>;
}
