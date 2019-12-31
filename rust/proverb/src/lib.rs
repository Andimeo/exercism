pub fn build_proverb(list: &[&str]) -> String {
    let mut ans = String::new();
    for i in 1..list.len() {
        ans.push_str(format!("For want of a {} the {} was lost.\n", list.get(i-1).unwrap(), list.get(i).unwrap()).as_str());
    }
    if !list.is_empty() {
        ans.push_str(format!("And all for the want of a {}.", list.first().unwrap()).as_str());
    }
    ans
}
