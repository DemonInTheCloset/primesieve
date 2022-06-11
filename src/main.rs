fn sieve<const LIMIT: usize>() -> Vec<u64> {
    let mut is_prime = vec![true; LIMIT];

    for ix in 0..LIMIT {
        if !is_prime[ix] {
            continue;
        }

        let num = 2 * ix + 3;

        if num * num > LIMIT {
            break;
        }

        for factor in (((num * num - 3) / 2)..LIMIT).step_by(num) {
            is_prime[factor] = false
        }
    }

    let mut primes: Vec<u64> = vec![2];

    for (ix, prime) in is_prime.iter().enumerate() {
        if *prime {
            primes.push((2 * ix + 3) as u64)
        }
    }

    primes
}

fn main() {
    println!("{:?}", sieve::<1000000000>())
}
