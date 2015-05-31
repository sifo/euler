fn problem9(n: usize) -> (usize, usize, usize) {
    for i in 1..(n/2) {
        for j in 1..(n/2) {
            for k in 1..(n/2) {
                if i*i + j*j == k*k && i + j + k == n {
                    return (i, j, k)
                }
            }
        }
    }
    (0, 0, 0)
}

#[test]
fn test_problem8() {
    assert_eq!((3, 4, 5), problem9(12));
}

#[allow(dead_code)]
fn main() {
    let s = 1000;
    let r = problem9(s);
    println!("The Pythagorean triplet for which a + b + c = {} and a² + b² = c ² is {:?}.", s, r)
}
