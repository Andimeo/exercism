pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit).filter(|x| factors.iter().filter(|&n| *n != 0).any(|&n| x % n == 0)).sum()
}
