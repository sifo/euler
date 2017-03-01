package main

import (
	"testing"
)

func TestIsPalindrome(t *testing.T) {
	p := [7]int{9009, 90009, 909, 99, 9, 19, 182}
	r := [7]bool{true, true, true, true, true, false, false}

	for i := 0; i < len(p); i++ {
		if IsPalindrome(p[i]) != r[i] {
			t.Error("This failed!")
		}
	}
}

func TestLargestPalindromeByNumberOfDigit(t *testing.T) {
	N := [1]int{2}
	X := [1]int{99}
	Y := [1]int{91}
	R := [1]int{9009}

	for i := 0; i < len(N); i++ {
		r, x, y := LargestPalindromeByNumberOfDigit(N[i])
		if r != R[i] || x != X[i] || y != Y[i] {
			t.Error("This failed!")
		}
	}
}

func TestIntervalByNumberOfDigit(t *testing.T) {
	N := [3]int{1, 2, 3}
	X := [3]int{1, 10, 100}
	Y := [3]int{9, 99, 999}

	for i := 0; i < len(N); i++ {
		x, y := IntervalByNumberOfDigit(N[i])
		if x != X[i] || y != Y[i] {
			t.Error("This failed!")
		}
	}
}
