package solver

import "strings"

type SolverOne struct {
}

func (solver SolverOne) SolutionOne(lines []string) int {
    answer := 0

    for _, element := range lines {
        answer += strings.Count(element, "(") - strings.Count(element, ")")
    }

    return answer
}

func (solver SolverOne) SolutionTwo(lines []string) int {
    position := 0
    line := lines[0]
    characters := strings.Split(line, "")

    for index, element := range characters {
        switch element {
        case "(":
            position++;
        case ")":
            position--;
        }

        if position == -1 {
            return index + 1
        }
    }
    panic("No number found")
}
