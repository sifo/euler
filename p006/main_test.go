package main

import "testing"

func TestSquareSum(t *testing.T) {
	X := [1]int{1}
	Y := [1]int{10}
	R := [1]int{3025}

	for i := 0; i < len(X); i++ {
		actual := SquareSum(X[i], Y[i])
		expected := R[i]
		if actual != expected {
			t.Errorf("This failed! %d != %d\n", actual, expected)
		}
	}
}

func TestSumSquares(t *testing.T) {
	X := [1]int{1}
	Y := [1]int{10}
	R := [1]int{385}

	for i := 0; i < len(X); i++ {
		actual := SumSquares(X[i], Y[i])
		expected := R[i]
		if actual != expected {
			t.Errorf("This failed! %d != %d\n", actual, expected)
		}
	}
}

func TestSumSquareDifference(t *testing.T) {
	X := [1]int{1}
	Y := [1]int{10}
	R := [1]int{2640}

	for i := 0; i < len(X); i++ {
		actual := SumSquareDifference(X[i], Y[i])
		expected := R[i]
		if actual != expected {
			t.Errorf("This failed! %d != %d\n", actual, expected)
		}
	}
}

func TestFastSumSquareDifference(t *testing.T) {
	X := [1]int{10}
	R := [1]int{2640}

	for i := 0; i < len(X); i++ {
		actual := FastSumSquareDifference(X[i])
		expected := R[i]
		if actual != expected {
			t.Errorf("This failed! %d != %d\n", actual, expected)
		}
	}
}
