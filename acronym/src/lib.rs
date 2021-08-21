use regex::Regex;

pub fn abbreviate(phrase: &str) -> String {
    // phrase
    //     .chars()
    //     .take(1)
    //     .chain(
    //         phrase
    //             .chars()
    //             .collect::<Vec<char>>()
    //             .windows(2)
    //             .filter(|w| {
    //                 (!w[0].is_alphabetic() && w[0] != '\'' && w[1].is_alphabetic()) || 
    //                 (!w[0].is_ascii_uppercase() && w[1].is_ascii_uppercase())
    //             })
    //             .map(|w| w[1].to_ascii_uppercase())
    //     )
    //     .collect()
    // phrase.split(&[' ', '-'][..])
    //     .flat_map(|w| {
    //         w.chars()
    //             .filter(|c| c.is_alphabetic())
    //             .take(1)
    //             .map(|c| c.to_ascii_uppercase())
    //     }).collect()

    let re = Regex::new(r"(\b[A-Z]+\b|[a-zA-Z][a-z]+)").unwrap();
    re.find_iter(phrase)
        .map(|m| m.as_str().chars().next().unwrap().to_ascii_uppercase())
        .collect()
}
