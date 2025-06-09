pub fn files(path: &str) -> Option<impl Iterator<Item = String>> {
    let entries = std::fs::read_dir(path).ok()?;
    let iter = entries.filter_map(|entry| {
        let path = entry.ok()?.path();
        std::fs::read_to_string(path).ok()
    });
    Some(iter)
}
