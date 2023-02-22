use text_io::scan;

fn disassembly_num(n: u128) -> Vec<(u128, u128)>
{
    let mut x = n;

    let mut primes = Vec::new();

    for i in 2..(n + 1) {
        if x == 1 {
            break
        }

        if x % i != 0 {
            continue
        }

        let mut times = 0;

        while x % i == 0 {
            times += 1;
            x /= i;
        }

        assert_ne!(i, 0);

        primes.push((i, times));
    }

    assert!(!primes.is_empty(), "primes is empty; n = {}", n);

    primes
}

fn main() {
    let n: u128;
    let k: u128;
    scan!("{} {}", n, k);

    let primes = disassembly_num(k);

    let zeroes: u128 = primes.iter().map(|(i, t)| {
        let i = *i;
        let t = *t;

        let mut x: u128 = i;

        assert_ne!(i, 0);

        let mut times = 0;

        while x <= n {
            assert_ne!(x, 0);

            times += n / x;

            let prev_x = x;

            x *= i;

            assert!(x > i, "x {} wasn't bigger than i {}; n = {}; n * i = {}; prev_x = {}", x, i, n, n * i, prev_x);
        }

        times /= t;

        times
    }).min().unwrap();

    println!("{}", zeroes);
}
