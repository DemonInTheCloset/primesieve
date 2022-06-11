#![feature(test)]

struct BitSet {
    data: Vec<u64>,
}

impl BitSet {
    // fn new() -> Self {
    //     Self { data: vec![] }
    // }

    fn with_capacity(capacity: usize) -> Self {
        Self {
            data: vec![0; (capacity + 63) / 64],
        }
    }

    // fn capacity(&self) -> usize {
    //     self.data.len() * 64
    // }

    const fn ix_and_mask(elem: usize) -> (usize, u64) {
        (elem / 64, 1 << (elem % 64))
    }

    fn add(&mut self, elem: usize) {
        let (ix, mask) = Self::ix_and_mask(elem);
        self.data[ix] |= mask;
    }

    // fn remove(&mut self, elem: usize) {
    //     let (ix, mask) = Self::ix_and_mask(elem);
    //     self.data[ix] &= !mask;
    // }

    fn contains(&self, elem: usize) -> bool {
        let (ix, mask) = Self::ix_and_mask(elem);
        return self.data[ix] & mask != 0;
    }
}

fn sieve<const LIMIT: usize>() -> Vec<u64> {
    let mut not_prime = BitSet::with_capacity(LIMIT);

    for ix in 0..LIMIT {
        if not_prime.contains(ix) {
            continue;
        }

        let num = 2 * ix + 3;

        if num * num > LIMIT {
            break;
        }

        for factor in (((num * num - 3) / 2)..LIMIT).step_by(num) {
            not_prime.add(factor);
        }
    }

    let mut primes = vec![2];

    for ix in 0..LIMIT {
        if not_prime.contains(ix) {
            continue;
        }
        primes.push((2 * ix + 3) as u64)
    }

    primes
}

fn main() {
    println!("{:?}", sieve::<1000000000>())
}

#[cfg(test)]
mod benchmarks {
    extern crate test;
    use test::Bencher;

    use crate::sieve;

    #[bench]
    fn bench_1m_primes(b: &mut Bencher) {
        b.iter(|| test::black_box(sieve::<1000000>()))
    }

    #[bench]
    fn bench_10m_primes(b: &mut Bencher) {
        b.iter(|| test::black_box(sieve::<10000000>()))
    }
}
