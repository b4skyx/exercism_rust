pub fn is_armstrong_number(num: u32) -> bool {
    let mut sum = 0;
    let temp = num.to_string();

    for ch in temp.chars(){
        sum = sum + ((ch.to_digit(10)).unwrap()).pow(temp.chars().count() as u32);
    }
    if sum == num{
        true
    }
    else{
        false
    }
}
