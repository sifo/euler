package main

import (
	"testing"
)

func TestLargestPrimeFactor(t *testing.T) {
	n := LargestPrimeFactor(13195)
	if n != 29 {
		t.Error("This failed!", n)
	}
}
