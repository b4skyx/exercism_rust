pub fn square_of_sum(n: u32) -> u32 {
    let mut res: u32 = 0;
    for i in 1..(n + 1) {
        res = res + i;
    }
    return res * res;
}

pub fn sum_of_squares(n: u32) -> u32 {
    if n == 1 {
        return 1;
    } else {
        return (n * n) + sum_of_squares(n - 1);
    }
}

pub fn difference(n: u32) -> u32 {
    return square_of_sum(n) - sum_of_squares(n);
}
