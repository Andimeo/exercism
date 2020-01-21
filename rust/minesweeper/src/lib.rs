pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.len() == 0 || minefield[0].len() == 0 {
        (0..minefield.len()).map(|_| String::from("")).collect()
    } else {
        let (n, m) = (minefield.len() as i32, minefield[0].len() as i32);
        let is_mine = |x, y| { minefield[x as usize].chars().nth(y as usize).unwrap() == '*' };
        let calc = |cx, cy| {
            if is_mine(cx, cy) {
                return '*';
            }
            let xs = std::cmp::max(0, cx - 1);
            let xe = std::cmp::min(n - 1, cx + 1);
            let ys = std::cmp::max(0, cy - 1);
            let ye = std::cmp::min(m - 1, cy + 1);
            let c = (xs..xe + 1)
                .flat_map(|x| (ys..ye + 1).map(move |y| (x, y)))
                .filter(|(x, y)| (*x != cx || *y != cy))
                .filter(|(x, y)| is_mine(*x, *y))
                .count();
            return if c == 0 {
                ' '
            } else {
                (c as u8 + b'0') as char
            };
        };
        (0..n)
            .map(|cx| (0..m).map(|cy| calc(cx, cy)).collect())
            .collect()
    }
}
