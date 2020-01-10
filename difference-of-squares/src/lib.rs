pub fn square_of_sum(n: u32) -> u32 {
    let mut res:u32 = 0;
    (1..(n+1)).for_each(|x| res=res+x);
    res*res
}

pub fn sum_of_squares(n: u32) -> u32 {
    if n>1{
        return n*n+sum_of_squares(n-1);
    }
    else{
        return 1;
    }
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n)-sum_of_squares(n)
}
