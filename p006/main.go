package main

import "fmt"

func SquareSum(x int, y int) int {
	res := 0
	for i := x; i <= y; i++ {
		res += i
	}
	return res * res
}

func SumSquares(x int, y int) int {
	res := 0
	for i := x; i <= y; i++ {
		res += i * i
	}
	return res
}

func SumSquareDifference(x int, y int) int {
	return SquareSum(x, y) - SumSquares(x, y)
}

func FastSumSquareDifference(n int) int {
	return ((n - 1) * n * (n + 1) * (3*n + 2)) / 12
}

func main() {
	n := FastSumSquareDifference(100)
	fmt.Printf("The difference between the sum of the squares of the first one hundred natural numbers and the square of the sum is %d.\n", n)
}
