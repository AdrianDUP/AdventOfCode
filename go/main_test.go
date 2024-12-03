package main

import (
	solver2024 "aoc/pkg/2024"
	"fmt"
	"testing"
)

func BenchmarkDay1(b *testing.B) {
    fileLines := loadFileContent("inputs/2024/day1.txt")
    solv := solver2024.SolverOne{}
    for i := 0; i < b.N; i++ {
        solv.SolutionTwo(fileLines)
    }
}

func TestRemoveElement(t *testing.T) {
    want1 := []int{
	1, 3, 6, 5, 8, 10,
    }

    want2 := []int{
	3, 6, 5, 8, 10,
    }

    want3 := []int{
	1, 6, 5, 8, 10,
    }

    base := []int{
	1, 3, 6, 5, 8, 10,
    }

    assert, message := compareArrays(want1, base, "Test 1")

    if !assert {
	t.Error(message)
    }

    got2 := solver2024.RemoveIndex(base, 0)

    assert, message = compareArrays(got2, want2, "Test 2")

    if !assert {
	t.Error(message)
    }

    got3 := solver2024.RemoveIndex(base, 1)

    assert, message = compareArrays(got3, want3, "Test 3")

    if !assert {
	t.Error(message)
    }
}

func compareArrays(expected []int, got []int, name string) (bool, string) {
    if len(expected) != len(got) {
	return false, fmt.Sprintf("%s: Got length %d, expected: %d", name, len(got), len(expected))
    }

    for index, element := range got {
	if element != expected[index] {
	    return false, fmt.Sprintf("%s: Got element %d, expected: %d", name, element, expected[index])
	}
    }

    return true, ""
}
