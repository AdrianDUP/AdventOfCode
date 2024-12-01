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
    return 1;
}
