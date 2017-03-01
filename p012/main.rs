// Help: http://mathschallenge.net/library/number/number_of_divisors

fn triangle_number(n: usize) -> usize {
    n * (n+1) / 2
}

fn number_of_divisors(mut n: usize) -> usize {
    let mut res = 0;
    let mut divisors = vec![];
    let mut d = 2;
    while n > 1 {
        while n%d == 0 {
            divisors.push(d);
            res += 1;
            n = n / d;
        }
        d += 1;
    }
    return res + divisors.len()
}

fn problem12(n: usize) -> usize {
    let mut res = 0;
    let mut i = 1;
    while number_of_divisors(res) <= n {
        //println!("res: {}", res);
        //println!("n: {}", number_of_divisors(res));
        res = triangle_number(i);
        i += 1;
    }
    res
}

#[test]
fn test_triangle_number() {
    assert_eq!(0, triangle_number(0));
    assert_eq!(55, triangle_number(10));
}

#[test]
fn test_number_of_divisors() {
    assert_eq!(4, number_of_divisors(10));
    assert_eq!(6, number_of_divisors(28));
    assert_eq!(10, number_of_divisors(48));
}

#[test]
fn test_problem12() {
    assert_eq!(28, problem12(5));
}

#[allow(dead_code)]
fn main() {
    let r = problem12(500);
    println!("The value of the first triangle number to have over five hundred divisors is {}.", r);
}
