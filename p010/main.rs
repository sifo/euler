pub fn problem10(n: usize) -> usize {
    if n <= 2 {
        return 0
    }
    let mut primes = vec![2, 3, 5, 7, 11];
    let mut res = primes[0];
    for i in 1..primes.len() {
        if primes[i] < n {
            res += primes[i];
        } else {
            return res
        }
    }
    while primes[primes.len() - 1] < n {
        let mut c = primes[primes.len()-1] + 2;
        let mut limit = (c as f64).sqrt() as usize + 1;
        loop {
            let mut is_prime = true;
            for i in 0..(primes.len() - 1) {
                if primes[i] > limit {
                    break;
                }
                if c % primes[i] == 0 {
                    is_prime = false;
                    break;
                }
            }
            if is_prime {
                break
            } else {
                c = c + 1;
                limit = (c as f64).sqrt() as usize + 1;
            }
        }
        primes.push(c);
        if c < n {
            res += c;
        }
    }
    return res
}

#[test]
fn test_problem10() {
    assert_eq!(17, problem10(10));
    assert_eq!(77, problem10(20));
}

#[allow(deadcode)]
fn main() {
    let r = problem10(2_000_000);
    println!("The sum of all primes below two million is {}.", r);
}
