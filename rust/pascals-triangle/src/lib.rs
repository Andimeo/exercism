pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut result = PascalsTriangle { rows: vec![] };
        for i in 0..row_count as usize {
            if i == 0 {
                result.rows.push(vec![1]);
            } else {
                result.rows.push(vec![]);
                for j in 0..i+1 {
                    let v = match j {
                        k if k == 0 => result.rows[i-1][0],
                        k if k == i => result.rows[i-1][i-1],
                        _ => result.rows[i-1][j-1] + result.rows[i-1][j],
                    };
                    result.rows[i].push(v);
                }
            }
        }
        result
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
