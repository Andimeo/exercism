pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let n = input.len();
    if n == 0 {
        return vec![];
    }
    let m = input[0].len();

    let mut max_in_row = vec![0; n];
    let mut min_in_column = vec![std::u64::MAX; m];

    for i in 0..n {
        for j in 0..m {
            max_in_row[i] = std::cmp::max(max_in_row[i], input[i][j]);
            min_in_column[j] = std::cmp::min(min_in_column[j], input[i][j]);
        }
    }
    let mut results = vec![];
    for i in 0..n {
        for j in 0..m {
            if input[i][j] >= max_in_row[i] && input[i][j] <= min_in_column[j] {
                results.push((i, j));
            }
        }
    }
    results
}
