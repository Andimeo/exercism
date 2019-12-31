pub fn nth(n: u32) -> u32 {
    let mut primes = Vec::<u32>::new();
    primes.push(2);
    let mut i = 3;

    loop {
        let mut is_prime = true;
        for p in &primes {
            if i % p == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(i);
            if primes.len() >= n as usize + 1 {
                break;
            }
        }
        i = i + 1;
    }
    *(primes.get(n as usize).unwrap())
}
