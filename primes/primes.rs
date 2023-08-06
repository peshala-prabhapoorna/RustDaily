fn main() {
    let mut primes = vec![];
    for i in 2..1000 {
        let mut is_prime = true;
        for j in 2..i {
            if i % j == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(i);
        }
    }
    for prime in primes {
        println!("{}", prime);
    }
}
