package solver2024

import (
	"fmt"
	"strconv"
	"strings"
)

type SolverTwo struct {
}

func (solver SolverTwo) SolutionOne(lines []string) int {
    reports := makeReportsFromLines(lines)
    answer := 0

    for _, element := range reports {
        if isSafeReport(element.numbers) {
            answer++
        }
    }

    return answer;
}

func (solver SolverTwo) SolutionTwo(lines []string) int {
    reports := makeReportsFromLines(lines)
    answer := 0

    for _, element := range reports {
        for i := -1; i < len(element.numbers); i++ {
            list := element.numbers
            if i >= 0 {
                list = RemoveIndex(list, i)
            }
            fmt.Printf("List: %v\n", list)
            safe := isSafeReport(list)

            if safe {
                answer++
                break
            }
        }
    }

    return answer;
}

type report struct {
    numbers []int
}

func (report report) isSafe() bool {
    var previousNumber int
    var isAscending bool

    for index, element := range report.numbers {
        if index == 0 {
            previousNumber = element
            continue
        }

        if index == 1 {
            if element > previousNumber {
                isAscending = true
            } else {
                isAscending = false
            }
        }

        if directionChanged(previousNumber, element, isAscending) || previousNumber == element {
            return false
        }

        difference := element - previousNumber

        if isAscending && difference > 3 || !isAscending && difference < -3 {
            return false
        }

        previousNumber = element
    }

    return true
}

func isSafeReport(numbers []int) bool {
    // fmt.Printf("Checking %v\n", numbers)
    var previousNumber int
    var isAscending bool

    for index, element := range numbers {
        // fmt.Printf("Current index: %d, element: %d\n", index, element)
        if index == 0 {
            previousNumber = element
            continue
        }

        if index == 1 {
            if element > previousNumber {
                isAscending = true
            } else {
                isAscending = false
            }
        }

        if !isSafeSequence(previousNumber, element, isAscending) {
            // fmt.Println("Not safe")
            return false
        }

        previousNumber = element
    }

    return true
}

func isSafeSequence(previousNumber int, nextNumber int, isAscending bool) bool {
    if directionChanged(previousNumber, nextNumber, isAscending) {
        return false
    }
    if previousNumber == nextNumber {
        return false
    }

    difference := nextNumber - previousNumber

    if isAscending && difference > 3 || !isAscending && difference < -3 {
        return false
    }

    return true
}

func directionChanged(previousNumber int, nextNumber int, isAscending bool) bool {
    if isAscending && previousNumber > nextNumber {
        return true
    } else if !isAscending && previousNumber < nextNumber {
        return true
    }

    return false
}

func makeReportsFromLines(lines []string) []report {
    reports := []report{}

    for _, element := range lines {
        reports = append(reports, makeReportFromLine(element))
    }

    return reports
}

func makeReportFromLine(line string) report {
    numbers := strings.Split(line, " ")

    structNumbers := []int{}

    for _, number := range numbers {
        newNumber, _ := strconv.Atoi(number)
        structNumbers = append(structNumbers, newNumber)
    }

    return report{
        structNumbers,
    }
}

func RemoveIndex(s []int, index int) []int {
    newSlice := []int{}
    newSlice = append(newSlice, s[:index]...)
    newSlice = append(newSlice, s[index+1:]...)
    return newSlice
}
