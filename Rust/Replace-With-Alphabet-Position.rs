fn alphabet_position(text: &str) -> String {
    // Code here...
    text.to_uppercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| (c as u32 - 'A' as u32 + 1).to_string())
        .collect::<Vec<_>>()
        .join(&" ")
}
