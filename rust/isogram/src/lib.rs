pub fn check(candidate: &str) -> bool {
    let mut alpha = vec![0; 26];
    candidate.chars().for_each(|c: char| {
        if !c.is_alphabetic() {
            return;
        }
        let s = if c.is_uppercase() { 'A' } else { 'a' };
        let pos = c as usize - s as usize;
        alpha[pos] += 1;
    });
    return alpha.iter().all(|count: &i32| { *count <= 1 });
}
