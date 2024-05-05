fn main() {
    // println!("Hello, world!");
    //
    let result = is_prime(12);
    println!("{}", result);
}

fn is_prime(n: i64) -> bool {
    if n <= 1 {
        return false;
    }

    for i in 2..=(n as f64).sqrt() as i64 {
        if n % i == 0 {
            return false;
        }
    }

    true
}
