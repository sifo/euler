#![feature(core)] extern crate core;

struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci {
        curr: 1,
        next: 1,
    }
}

pub fn problem2_bis(l: u32) -> u32 {
    let sum = fibonacci()
        .filter(|&v| v % 2 == 0)
        .take_while(|&v| v <= l)
        .sum();
    sum
}

pub fn problem2(l: u32) -> u32 {
    let mut a = 1;
    let mut b = 2; 
    let mut c = a + b;
    let mut s = b;
    while c <= l {
        c = a + b;
        a = b;
        b = c;
        if c % 2 == 0 {
            s += c
        }
    }
    s
}
    
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_problem2() {
        assert_eq!(problem2(4_000_000), 4613732);
    }

    #[test]
    fn test_problem2_bis() {
        assert_eq!(problem2_bis(4_000_000), 4613732);
    }
}

fn main() {
    let l = 4_000_000;
    println!("fib({}): {}", l, problem2(l));
    println!("fib({}): {}", l, problem2_bis(l));
}
