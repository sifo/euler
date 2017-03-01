
pub fn prime(n: usize) -> usize {
    let mut primes = vec![2, 3, 5, 7, 11];
    if n < primes.len() {
        return primes[n - 1]
    } 
    let mut nbis = primes.len();
    while nbis < n {
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
        nbis = nbis + 1;
    }
    return primes[primes.len() - 1]
}

#[test]
fn test_prime() {
    assert_eq!(11, prime(5));
    assert_eq!(13, prime(6));
    assert_eq!(17, prime(7));
    assert_eq!(19, prime(8));
    assert_eq!(23, prime(9));
}

fn main() {
    println!("The 10 001st prime number is {}.", prime(10_001))
}
