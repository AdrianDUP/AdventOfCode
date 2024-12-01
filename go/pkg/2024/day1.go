package solver2024

import (
	"fmt"
	"slices"
	"strconv"
	"strings"
)

type SolverOne struct {
}

func (solver SolverOne) SolutionOne(lines []string) int {
    answer := 0

    leftSide := []int{}
    rightSide := []int{}

    for _, element := range lines {
        numbers := strings.Split(element, " ")
        leftNumber, err := strconv.Atoi(numbers[0])
        if  err != nil {
            panic(fmt.Sprintf("Number %s is not valid", numbers[0]))
        }
        rightNumber, err := strconv.Atoi(numbers[len(numbers)-1])
        if  err != nil {
            panic(fmt.Sprintf("Number %s is not valid", numbers[len(numbers)-1]))
        }

        leftSide = append(leftSide, leftNumber)
        rightSide = append(rightSide, rightNumber)
    }

    slices.Sort(leftSide)
    slices.Sort(rightSide)

    for index, element := range leftSide {
        distance := element - rightSide[index]
        if (distance < 0) {
            distance = distance * -1
        }

        answer += distance
    }

    return answer;
}

func (solver SolverOne) SolutionTwo(lines []string) int {
    answer := 0

    leftSide := []int{}
    rightSide := []int{}

    for _, element := range lines {
        numbers := strings.Split(element, " ")
        leftNumber, err := strconv.Atoi(numbers[0])
        if  err != nil {
            panic(fmt.Sprintf("Number %s is not valid", numbers[0]))
        }
        rightNumber, err := strconv.Atoi(numbers[len(numbers)-1])
        if  err != nil {
            panic(fmt.Sprintf("Number %s is not valid", numbers[len(numbers)-1]))
        }

        leftSide = append(leftSide, leftNumber)
        rightSide = append(rightSide, rightNumber)
    }

    counts := make(map[int]int)

    for _, element := range rightSide {
        if val, ok := counts[element]; ok {
            counts[element] = val + 1;
        } else {
            counts[element] = 1
        }
    }

    for _, element := range leftSide {
        if val, ok := counts[element]; ok {
            answer += val * element
        } 
    }

    return answer
}
