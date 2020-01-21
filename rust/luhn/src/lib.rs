/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .filter(|&c| !c.is_whitespace())
        .rev()
        .zip((0..2).cycle())
        .try_fold((0, 0), |(num, acc), (c, i)| match c {
            c if c < '0' || c > '9' => None,
            c => {
                let x = c as u32 - '0' as u32;
                Some((num + 1, acc + x + i * x - i * (x >= 5) as u32 * 9))
            }
        })
        .map_or(false, |(num, acc)| acc % 10 == 0 && num > 1)
}
