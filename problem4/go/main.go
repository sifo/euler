package main

import (
	"fmt"
	"strconv"
)

func IsPalindrome(n int) bool {
	s := strconv.Itoa(n)
	for i := 0; i <= len(s)/2; i++ {
		if s[i] != s[len(s)-1-i] {
			return false
		}
	}
	return true
}

func IntervalByNumberOfDigit(digit int) (int, int) {
	min := "1"
	max := "9"

	for i := 1; i < digit; i++ {
		min += "0"
		max += "9"
	}
	rmin, _ := strconv.Atoi(min)
	rmax, _ := strconv.Atoi(max)
	return rmin, rmax
}

func LargestPalindromeByNumberOfDigit(digit int) (int, int, int) {
	res := 0
	x := 0
	y := 0
	min, max := IntervalByNumberOfDigit(digit)
	for i := min; i <= max; i++ {
		for j := min; j <= max; j++ {
			p := i * j
			if IsPalindrome(p) {
				res = p
				x = i
				y = j
			}
		}
	}
	return res, x, y
}

func main() {
	n, x, y := LargestPalindromeByNumberOfDigit(3)
	fmt.Printf("The largest palindrome made from the product of two 3-digit numbers is %d = %d * %d.\n", n, x, y)
}
