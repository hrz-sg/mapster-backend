pub (crate) fn generate_direct_chat_key(a: &str, b: &str) -> String {
    if a < b {
        format!("{}:{}", a, b)
    } else {
        format!("{}:{}", b, a)
    }
}