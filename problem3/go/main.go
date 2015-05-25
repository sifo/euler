package main

import (
	"fmt"
)

func LargestPrimeFactor(n int) int {
	factors := make([]int, 1)
	d := 2
	for n > 1 {
		for n%d == 0 {
			factors = append(factors, d)
			n = n / d
		}
		d = d + 1
	}
	return factors[len(factors)-1]
}

func main() {
	n := 600851475143
	fmt.Printf("Largest prime factor of %d is %d.\n", n, LargestPrimeFactor(n))
}
