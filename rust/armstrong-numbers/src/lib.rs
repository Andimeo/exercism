pub fn is_armstrong_number(num: u32) -> bool {
    let s = num.to_string();
    let len = s.len() as u32;
    s.chars().map(|c| (c as u32 - '0' as u32).pow(len)).sum::<u32>() == num
}
