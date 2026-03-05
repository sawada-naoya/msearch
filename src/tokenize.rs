use regex::Regex;

pub fn tokenize(text: &str) -> Vec<String> {
    // MVP: ASCII-ish tokenizer. Lowercase + non-alnum => spaces.
    // NOTE: This is intentionally simplistic; Japanese tokenization is out of scope for MVP.
    static RE: once_cell::sync::Lazy<Regex> =
        once_cell::sync::Lazy::new(|| Regex::new(r"[^0-9A-Za-z]+").unwrap());

    let lowered = text.to_lowercase();
    RE.split(&lowered)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}
