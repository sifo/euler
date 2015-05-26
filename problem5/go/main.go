package main

import "fmt"

func SmallestNumberDividedByInterval(x int, y int) int {
	res := y
	ok := false
	for !ok {
		ok = true
		for i := x; i <= y; i++ {
			if res%i != 0 {
				ok = false
				res = res + 1
				break
			}
		}
	}
	return res
}

func main() {
	n := SmallestNumberDividedByInterval(1, 20)
	fmt.Printf("The smallest positive number that is evenly divisible by all of the numbers from 1 to 20 is %d.\n", n)
}
