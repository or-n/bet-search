use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub async fn save<P>(content: &[u8], path: P) -> std::io::Result<()>
where
    P: AsRef<std::path::Path>,
{
    File::create(path).await?.write_all(content).await
}
