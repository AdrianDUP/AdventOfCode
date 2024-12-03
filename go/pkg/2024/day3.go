package solver2024

import (
	"regexp"
	"strconv"
)

type SolverThree struct {
}

func (solver SolverThree) SolutionOne(lines []string) int {
    answer := 0

    for _, line := range lines {
        if line == "" {
            continue
        }

        multiplications := findMultiplications(line)

        for _, multiplication := range multiplications {
            answer += multiplyElement(multiplication)
        }
    }
    return answer
}

func (solver SolverThree) SolutionTwo(lines []string) int {
    answer := 0
    enabled := true

    for _, line := range lines {
        if line == "" {
            continue
        }

        instructions := findExtendedInstructions(line)

        for _, instruction := range instructions {
            switch instruction {
            case "do()":
                enabled = true
            case "don't()":
                enabled = false
            default:
                if enabled {
                    answer += multiplyElement(instruction)
                } else {
                    continue
                }
            }
        }
    }
    return answer
}

func findMultiplications(line string) []string {
    regex, err := regexp.Compile(`mul\([0-9]{1,3},[0-9]{1,3}\)`)
    if err != nil {
        panic("Invalid Regex")
    }

    return regex.FindAllString(line, -1)
}

func findExtendedInstructions(line string) []string {
    regex, err := regexp.Compile(`(mul\([0-9]{1,3},[0-9]{1,3}\))|(do\(\))|(don't\(\))`)
    if err != nil {
        panic("Invalid Regex")
    }

    return regex.FindAllString(line, -1)
}

func multiplyElement(element string) int {
    regex, err := regexp.Compile(`[0-9]{1,3}`)

    if err != nil {
        panic("Invalid Regex 2")
    }

    numbers := regex.FindAllString(element, -1)

    if len(numbers) != 2 {
        panic("2 numbers not found")
    }

    num1, _ := strconv.Atoi(numbers[0])
    num2, _ := strconv.Atoi(numbers[1])

    return num1 * num2
}
