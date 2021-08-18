use regex::Regex;

pub fn reply(message: &str) -> &str {
    let q = Regex::new(r"^.*\?\s*$").unwrap();
    let y = Regex::new(r"^[^a-z]+$").unwrap();
    let u = Regex::new(r"[A-Z]+").unwrap();
    let yq = Regex::new(r"^[^a-z]+\?$").unwrap();
    let s = Regex::new(r"^\s*$").unwrap();
    match message {
        msg if s.is_match(msg) => "Fine. Be that way!",
        msg if yq.is_match(msg) && u.is_match(msg) => "Calm down, I know what I'm doing!",
        msg if q.is_match(msg) => "Sure.",
        msg if y.is_match(msg) && u.is_match(msg) => "Whoa, chill out!",
        _ => "Whatever."
    }
}
