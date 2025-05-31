pub trait ToDBRecord {
    fn to_db_record(&self) -> Option<String>;
}

pub fn sanitize(x: &str) -> String {
    x.chars()
        .map(|c| match c {
            'ą' => 'a',
            'ć' => 'c',
            'ę' => 'e',
            'ł' => 'l',
            'ń' => 'n',
            'ó' => 'o',
            'ś' => 's',
            'ź' => 'z',
            'ż' => 'z',
            'Ą' => 'A',
            'Ć' => 'C',
            'Ę' => 'E',
            'Ł' => 'L',
            'Ń' => 'N',
            'Ó' => 'O',
            'Ś' => 'S',
            'Ź' => 'Z',
            'Ż' => 'Z',
            ' ' => '_',
            '-' => '_',
            'á' => 'a',
            _ => c,
        })
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect()
}
