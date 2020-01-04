pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }
    if n == 1 {
        return 3;
    }
    let mut count: u32 = 1;
    for r in 4..(u32::max_value()) {
        if is_prime(r) {
            count = count + 1;
        }
        if count == n {
            return r;
        }
    }
    return n;
}

pub fn is_prime(k: u32) -> bool {
    for r in 2..((k as f64).sqrt() as u32 + 1) {
        if k % r == 0 {
            return false;
        }
    }
    return true;
}
