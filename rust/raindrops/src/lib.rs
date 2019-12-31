pub fn raindrops(n: u32) -> String {
    let mut ans: String = String::new();
    if n % 3 == 0 {
        ans.push_str("Pling");
    }
    if n % 5 == 0 {
        ans.push_str("Plang");
    }
    if n % 7 == 0 {
        ans.push_str("Plong");
    }
    if ans.is_empty() {
        ans = n.to_string();
    }
    ans
}
