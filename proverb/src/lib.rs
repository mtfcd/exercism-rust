
pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        return "".into()
    }
    list.windows(2)
        .map(|w| {
            format!("For want of a {} the {} was lost.", w[0], w[1])
        })
        .reduce(|s1, s2| s1 + "\n" + &s2).unwrap()
    + &format!("\nAnd all for the want of a {}.", list[0])
}
