pub trait ToDBRecord {
    fn to_db_record(&self) -> Option<String>;
}
