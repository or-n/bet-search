pub trait Download<Client, Data>: Sized {
    type Error;

    async fn download(
        client: &mut Client,
        data: Data,
    ) -> Result<Self, Self::Error>;
}
