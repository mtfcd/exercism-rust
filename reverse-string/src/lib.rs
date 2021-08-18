use unicode_normalization::UnicodeNormalization;

pub fn reverse(input: &str) -> String {
    input
        .nfc()
        .collect::<Vec<char>>()
        .iter()
        .rev()
        .collect::<String>()
        .nfd()
        .to_string()
}
