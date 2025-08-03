use scraper::{ElementRef, Node};

pub fn clean_text(texts: scraper::element_ref::Text) -> String {
    texts
        .map(|t| t.trim())
        .filter(|t| !t.is_empty())
        .collect::<Vec<&str>>()
        .join(" ")
        .replace('\u{a0}', " ")
}

pub fn main_text(element: ElementRef) -> String {
    element
        .first_child()
        .and_then(|node| {
            if let Node::Text(text) = node.value() {
                Some(text.trim().to_string())
            } else {
                None
            }
        })
        .unwrap_or_default()
}
