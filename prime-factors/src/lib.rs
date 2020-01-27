pub fn factors(n: u64) -> Vec<u64> {
    let mut factors : Vec<u64> = Vec::new();
    let mut res: u64 = n;
    if n < 2{
        return factors;}
    if n == 2{
        factors.push(2);
        return factors;
    }
    loop{
        for i in 2..n{
            if res%i == 0{
                factors.push(i);
                res = res / i;
                break;
            }}
        if res == 1{
            break;
        }
    }
    factors
}
