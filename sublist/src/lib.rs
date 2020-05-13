#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }
    let res: Comparison;
    let cmp = |x: &[T], y: &[T]| -> bool {
        if x.len() == 0 {
            return true;
        }
        for a in y.windows(x.len()) {
            if a == x {
                return true;
            }
        }
        false
    };
    let (len1, len2) = (first_list.len(), second_list.len());
    if len1 == 0 && len2 == 0 {
        return Comparison::Equal;
    } else if len1 < len2 {
        if cmp(first_list, second_list) {
            res = Comparison::Sublist;
        } else {
            res = Comparison::Unequal;
        }
    } else {
        if cmp(second_list, first_list) {
            res = Comparison::Superlist;
        } else {
            res = Comparison::Unequal;
        }
    }
    return res;
}
