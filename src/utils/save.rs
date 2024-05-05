use std::io::Write;

pub fn save<P>(content: &[u8], path: P) -> std::io::Result<()>
where
    P: AsRef<std::path::Path>,
{
    std::fs::File::create(path)?.write_all(content)
}
