pub fn factors(n: u64) -> Vec<u64> {
    let mut ans = Vec::<u64>::new();
    let mut nn = n;
    let mut i = 2;
    while i * i <= nn && nn > 1 {
        while nn % i == 0 {
            nn /= i;
            ans.push(i);
        }
        i += 1;
    }
    if nn > 1 {
        ans.push(nn);
    }
    ans
}
