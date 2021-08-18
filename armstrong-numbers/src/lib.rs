pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits = vec![];
    let mut quo = num;
    while quo > 0 {
        let rem = quo % 10;
        digits.push(rem);
        quo /= 10;
    }

    let l = digits.len();
    num == digits.iter().map(|i| i.pow(l as u32)).sum()
}
