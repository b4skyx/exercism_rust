pub fn square(s: u32) -> u64 {
    if s == 0 || s > 64{
        panic!("Square must be between 1 and 64");}
    2_u64.pow(s-1)
}

pub fn total() -> u64 {
    let mut result : u64 = 0;
    (1..65).for_each(|x| result=result+square(x));
    return result;
}
