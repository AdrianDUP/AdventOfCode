package solver2023

import (
	"fmt"
	"regexp"
	"strconv"
)

type SolverOne struct {
}

func (solver SolverOne) SolutionOne(lines []string) int {
    answer := 0
    regExp, _ := regexp.Compile("[0-9]")

    for _, element := range lines {
        if element == "" {
            continue
        }

        numbers := regExp.FindAllString(element, -1)

        if len(numbers) == 0 {
            panic("No numbers found")
        }

        number := fmt.Sprintf("%s%s", numbers[0], numbers[len(numbers)-1])
        convertedNumber, err := strconv.Atoi(number)

        if err != nil {
            panic("Could not make number")
        }

        answer += convertedNumber
    }

    return answer;
}

func (solver SolverOne) SolutionTwo(lines []string) int {
    answer := 0
    regExp, _ := regexp.Compile("[0-9]|one|two|three|four|five|six|seven|eight|nine|zero")

    for _, element := range lines {
        if element == "" {
            continue
        }

        numbers := regExp.FindAllString(element, -1)

        if len(numbers) == 0 {
            panic("No numbers found")
        }

        firstNumber := solver.nameToNumber(numbers[0])
        lastNumber := solver.nameToNumber(numbers[len(numbers) - 1])

        number := fmt.Sprintf("%s%s", firstNumber, lastNumber)
        convertedNumber, err := strconv.Atoi(number)

        if err != nil {
            panic("Could not make number")
        }

        answer += convertedNumber
    }

    return answer
}

func (solver SolverOne) processLine(line string) int {
    regExp, _ := regexp.Compile("[0-9]")

    numbers := regExp.FindAllString(line, -1)

    if len(numbers) == 0 {
        panic("No numbers found")
    }

    firstNumber := numbers[0]
    lastNumber := numbers[len(numbers) - 1]

    number := fmt.Sprintf("%s%s", firstNumber, lastNumber)
    convertedNumber, err := strconv.Atoi(number)

    if err != nil {
        panic("Could not make number")
    }

    return convertedNumber
}

func (solver SolverOne) nameToNumber(number string) string {
    switch number {
    case "one":
    number = "1"
    case "two":
        number = "2"
    case "three":
    number = "3"
    case "four":
        number = "4"
    case "five":
        number = "5"
    case "six":
        number = "6"
    case "seven":
        number = "7"
    case "eight":
        number = "8"
    case "nine":
        number = "9"
    case "zero":
        number = "0"
    }

    return number
}
