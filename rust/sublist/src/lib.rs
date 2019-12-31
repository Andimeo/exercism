#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let (l1, l2) = (first_list.len(), second_list.len());
    let (short, long) = if l1 > l2 {
        (second_list, first_list)
    } else {
        (first_list, second_list)
    };
    let sl = short.len();
    if sl == 0 || long.windows(sl).any(|x| x == short) {
        match l1 as i32 - l2 as i32 {
            0 => Comparison::Equal,
            v if v < 0 => Comparison::Sublist,
            _ => Comparison::Superlist,
        }
    } else {
        Comparison::Unequal
    }
}