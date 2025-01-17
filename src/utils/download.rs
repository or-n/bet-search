pub trait Download {
    type Output;
    type Error;

    async fn download(&self) -> Result<Self::Output, Self::Error>;
}
