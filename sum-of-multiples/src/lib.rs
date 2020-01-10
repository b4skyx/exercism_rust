pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut result:u32 = 0;
    let mut fct:bool=true;
    for i in 1..limit{
        for r in factors.iter().filter(|x| x!=(&&0u32)){
            if i%r == 0 {
                result=result+i;
                break;
            }
        }
    }
    return result;
}
