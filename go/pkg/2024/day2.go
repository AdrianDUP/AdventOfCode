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
        if element.isSafeTemp() {
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
            safe, _ := isSafeReport(list)

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

func (report report) isSafeTemp() bool {
    safe, _ := isSafeReport(report.numbers)
    return safe
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

func (report report) isSafe2() bool {
    var previousNumber int
    previousPreviousNumber := 0
    var isAscending bool
    elementSkipped := false
    skippedElementChecked := false
    var skippedElement int

    for index, element := range report.numbers {
        // fmt.Printf("Checking element %d with previous element %d\n", element, previousNumber)
        if index == 0 {
            // fmt.Printf("Index 0\n")
            previousNumber = element
            continue
        }

        if index == 1 {
            // fmt.Println("Index 1")
            if element > previousNumber {
                // fmt.Println("Is Ascending")
                isAscending = true
            } else {
                // fmt.Println("Is Descending")
                isAscending = false
            }
        }

        isSafe := isSafeSequence(previousNumber, element, isAscending)

        if (!isSafe) {
            // fmt.Println("Not Safe")
            if !elementSkipped {
                // fmt.Println("No element skipped")
                if previousPreviousNumber != 0 && isSafeSequence(previousPreviousNumber, element, isAscending) {
                    // fmt.Println("Check 1")
                    elementSkipped = true
                    skippedElementChecked = true
                    previousNumber = element
                    continue
                } else {
                    skippedElement = element
                    elementSkipped = true
                }
            } 
            if elementSkipped && !skippedElementChecked {
                if isSafeSequence(skippedElement, element, isAscending) {
                    previousNumber = element
                    skippedElementChecked = true
                    continue
                } else {
                    return false
                }
            } else if !elementSkipped {
                skippedElement = element
                elementSkipped = true
                continue
            } else {
                return false
            }
        }

        previousNumber = element
    }

    return true
}

func isSafeReport(numbers []int) (bool, int) {
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
            return false, index
        }

        previousNumber = element
    }

    return true, 0
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
    return append(s[:index], s[index+1:]...)
}
