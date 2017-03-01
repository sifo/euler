
fn multiples(a: i32, b: i32, c: i32 ) -> i32 {
    let mut res = 0;
    for i in 1..c {
        if i % a == 0 || i % b == 0 {
            res = res + i;
        }
    }
    res
}

#[test]
fn test_multiples() {
    assert_eq!(multiples(3, 5, 10), 23);
}


fn multiples_vec(list: Vec<i32>, limit: i32) -> i32 {
    let mut res = 0;
    for i in 1..limit {
        for j in list.iter() {
            if i % *j == 0 {
                res = res + i;
            }
        }
    }
    res
}

#[test]
fn test_multiples_vec() {
    let v = vec![3, 5];
    assert_eq!(multiples_vec(v, 10), 23);
}

fn multiples_array(a: &[i32], limit: i32) -> i32 {
    let mut res = 0;
    for i in 1..limit {
        for j in a.iter() {
            if i % *j == 0 {
                res = res + i;
            }
        }
    }
    res
}

#[test]
fn test_multiples_array() {
    let array: [i32; 2] = [3, 5];
    assert_eq!(multiples_array(&array, 10), 23);
}

fn is_div(a: &[i32], n: i32) -> bool {
    for i in a.iter() {
        if n % *i == 0 {
            return true
        }
    }
    false
}

#[test]
fn test_is_div() {
    let array: [i32; 2] = [3, 5];
    assert!(is_div(&array, 3));
    assert!(!is_div(&array, 1));
}

fn multiples_functional(a: &[i32], limit: i32) -> i32 {
    let l = (1..limit).filter(|n| is_div(a, *n));
    let mut res = 0;
    for i in l {
        res = res + i;
    }
    res
}

#[test]
fn test_multiples_functional() {
    let array: [i32; 2] = [3, 5];
    assert_eq!(multiples_functional(&array, 10), 23);
}

fn is_div_mini(a: &[i32], i: i32) -> bool {
    a.iter().filter(|n| i % **n == 0).count() != 0
}

#[test]
fn test_is_div_mini() {
    let array: [i32; 2] = [3, 5];
    assert!(is_div_mini(&array, 3));
    assert!(!is_div_mini(&array, 1));
}

fn multiples_functional_mini(a: &[i32], limit: i32) -> i32 {
    (1..limit)
        .filter(|n| is_div_mini(a, *n))
        .fold(0, |s, x| s + x)
}

#[test]
fn test_multiples_functional_mini() {
    let array: [i32; 2] = [3, 5];
    assert_eq!(multiples_functional_mini(&array, 10), 23);
}

fn multiples_mini(a: &[i32], limit: i32) -> i32 {
    fn is_div(a: &[i32], i: i32) -> bool {
        a.iter().filter(|n| i % **n == 0).count() != 0
    }
    (1..limit)
        .filter(|n| is_div(a, *n))
        .fold(0, |s, x| s + x)
}

#[test]
fn test_multiples_mini() {
    let array = [3, 5];
    assert_eq!(multiples_mini(&array, 10), 23);
}

fn main() {
    println!("Hello!");
}
