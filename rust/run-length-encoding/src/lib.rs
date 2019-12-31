pub fn encode(source: &str) -> String {
    let mut segs = vec![];
    for (i, c) in source.char_indices() {
        if i == 0 || c != source.chars().nth(i - 1).unwrap() {
            segs.push(1);
        } else {
            *segs.last_mut().unwrap() += 1;
        }
    }
    let mut ans = String::from("");
    let mut acc = 0;
    for v in segs {
        if v > 1 {
            ans.push_str(&v.to_string()[..]);
        }
        ans.push(source.chars().nth(acc).unwrap());
        acc += v;
    }
    ans
}

pub fn decode(source: &str) -> String {
    let mut count = 0;
    let mut ans = String::from("");
    source.chars().for_each(|c: char| {
        if !c.is_digit(10) {
            ans.push_str(&vec![c; std::cmp::max(1, count)].iter().collect::<String>()[..]);
            count = 0;
        } else {
            count = count * 10 + (c as usize - '0' as usize);
        }
    });
    ans
}
