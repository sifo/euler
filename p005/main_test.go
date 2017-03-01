package main

import "testing"

func TestSmallestNumberDividedByInterval(t *testing.T) {
	X := [1]int{1}
	Y := [1]int{10}
	R := [1]int{2520}
	for i := 0; i < len(X); i++ {
		actual := SmallestNumberDividedByInterval(X[i], Y[i])
		expected := R[i]
		if actual != expected {
			t.Errorf("This failed! %d != %d.\n", actual, expected)
		}
	}
}
