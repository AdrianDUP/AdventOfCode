package solver2023

import (
	"fmt"
	"strconv"
	"strings"
)

type SolverTwo struct {
}

func (solver SolverTwo) SolutionOne(lines []string) int {
    answer := 0

    for _, line := range lines {
        theGame := makeGameFromLine(line)

        if theGame.isPossible(12, 14, 13) {
            answer += theGame.gameNumber
        }
    }

    return answer
}

func (solver SolverTwo) SolutionTwo(lines []string) int {
    answer := 0

    for _, line := range lines {
        theGame := makeGameFromLine(line)

        fmt.Printf("Game number %d power is %d\n", theGame.gameNumber, theGame.power())

        answer += theGame.power()
    }

    return answer
}

func makeGameFromLine(line string) game {
    stringParts := strings.Split(line, ": ")

    if len(stringParts) != 2 {
        panic("Invalid Split")
    }

    return game{
        makeGameNumber(stringParts[0]),
        makeGameRolls(stringParts[1]),
    }
}

func makeGameNumber(gameTag string) int {
    part := strings.Split(gameTag, " ")

    if len(part) != 2 {
        panic("Invalid game tag")
    }

    gameNumber, _ := strconv.Atoi(part[1])

    return gameNumber
}

func makeGameRolls(rolls string) []roll {
    rollStrings := strings.Split(rolls, "; ")

    rollStructs := []roll{}

    for _, element := range rollStrings {
        rollStructs = append(rollStructs, makeGameRoll(element))
    }

    return rollStructs
}

func makeGameRoll(rollValue string) roll {
    diceRolls := strings.Split(rollValue, ", ")

    red := 0
    blue := 0
    green := 0

    for _, dice := range diceRolls {
        elements := strings.Split(dice, " ")

        switch elements[1] {
        case "blue":
            blue, _ = strconv.Atoi(elements[0])
        case "green":
            green, _ = strconv.Atoi(elements[0])
        case "red":
            red, _ = strconv.Atoi(elements[0])
        }
    }

    return roll{
        red,
        blue,
        green,
    }
}

type game struct {
    gameNumber int
    rolls []roll
}

func (game game) isPossible(red int, blue int, green int) bool {
    for _, element := range game.rolls {
        if element.red > red || element.blue > blue || element.green > green {
            return false
        }
    }

    return true
}

func (game game) power() int {
    minimumRed := 0
    minimumBlue := 0
    minimumGreen := 0

    for _, element := range game.rolls {
        if (minimumRed == 0 || element.red > minimumRed) && element.red != 0 {
            minimumRed = element.red
        }

        if (minimumBlue == 0 || element.blue > minimumBlue) && element.blue != 0 {
            minimumBlue = element.blue
        }

        if (minimumGreen == 0 || element.green > minimumGreen) && element.green != 0 {
            minimumGreen = element.green
        }
    }

    return minimumRed * minimumBlue * minimumGreen
}

type roll struct {
    red int
    blue int
    green int
}
