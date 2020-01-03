pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }
    if n == 1 {
        return 3;
    }
    let mut count: u32 = 1;
    let mut current: u32 = 3;
    loop {
        current = current + 1;
        let mut prime: bool = true;
        for r in 2..(current / 2) {
            if current % r == 0 {
                prime = false;
                break;
            }
        }
        if prime == true {
            if count == n {
                return current;
            }
            count = count + 1;
        }
    }
}
fn is_prime (k: u32) -> u32 {
    if k <= 2 {
        false
    }
    for r in 2..(std::num::sqrt(k)
