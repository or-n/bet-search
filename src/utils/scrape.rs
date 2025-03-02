use scraper::{ElementRef, Node};

pub fn clean_text(texts: scraper::element_ref::Text) -> String {
    texts
        .map(|t| t.trim())
        .filter(|t| !t.is_empty())
        .collect::<Vec<&str>>()
        .join(" ")
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

pub fn split2(x: String, on: &str) -> Option<[String; 2]> {
    let parts: Vec<_> = x.split(on).collect();
    if parts.len() != 2 {
        return None;
    }
    let a = parts[0].to_string();
    let b = parts[1].to_string();
    Some([a, b])
}
