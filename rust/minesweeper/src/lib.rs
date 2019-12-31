pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.len() == 0 || minefield[0].len() == 0 {
        (0..minefield.len()).map(|_| String::from("")).collect()
    } else {
        let (n, m) = (minefield.len() as i32, minefield[0].len() as i32);
        let calc = |cx, cy| {
            if minefield[cx as usize].chars().nth(cy as usize).unwrap() == '*' {
                return '*';
            }
            let dx = [-1, -1, -1, 0, 0, 1, 1, 1];
            let dy = [-1, 0, 1, -1, 1, -1, 0, 1];
            let mut num_mines = b'0';
            for i in 0..8 {
                let nx = dx[i] + cx;
                let ny = dy[i] + cy;
                if nx >= 0
                    && nx < n as i32
                    && ny >= 0
                    && ny < m as i32
                    && minefield[nx as usize].chars().nth(ny as usize).unwrap() == '*'
                {
                    num_mines += 1;
                }
            }
            return if num_mines == b'0' { ' ' } else { num_mines as char };
        };
        (0..n)
            .map(|cx| (0..m).map(|cy| calc(cx, cy)).collect())
            .collect()
    }
}
