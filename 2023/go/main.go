package main

import (
	"aoc/2023/pkg/solver"
	"bufio"
	"fmt"
	"os"
	"strconv"
)

var (
solvers map[int]solver.Solver
    day int
    part int
    filePath string
    errVar error
)

func main() {
    if len(os.Args) != 4 {
        panic("missing arguments")
    }

    for index, element := range os.Args {
        fmt.Printf("Parsing arg index %d element %s\n", index, element)
        switch index {
        case 1:
            day, errVar = strconv.Atoi(element)
            if errVar != nil {
                panic("invalid day")
            }
            fmt.Printf("Day is %d", day)
        case 2:
            part, errVar = strconv.Atoi(element)
            if errVar != nil {
                panic("Invalid part")
            }
        case 3:
            filePath = element
        }
    }

    lines := loadFileContent(filePath)
    initSolvers()

    solv, ok := solvers[day]

    if !ok {
        panic("No solver for day")
    }

    var answer int
    switch part {
    case 1:
        answer = solv.SolutionOne(lines)
    case 2:
        answer = solv.SolutionTwo(lines)
    default:
        panic("Invalid part number")
    }

    fmt.Printf("Answer for day %d part %d is %d", day, part, answer)
}

func initSolvers() {
    solvers = make(map[int]solver.Solver)
    solvers[1] = solver.SolverOne{}
    solvers[2] = solver.SolverTwo{}
    solvers[3] = solver.SolverThree{}
    solvers[4] = solver.SolverFour{}
    solvers[5] = solver.SolverFive{}
    solvers[6] = solver.SolverSix{}
    solvers[7] = solver.SolverSeven{}
    solvers[8] = solver.SolverEight{}
    solvers[9] = solver.SolverNine{}
    solvers[10] = solver.SolverTen{}
    solvers[11] = solver.SolverEleven{}
    solvers[12] = solver.SolverTwelve{}
    solvers[13] = solver.SolverThirteen{}
    solvers[14] = solver.SolverFourteen{}
    solvers[15] = solver.SolverFifteen{}
    solvers[16] = solver.SolverSixteen{}
    solvers[17] = solver.SolverSeventeen{}
    solvers[18] = solver.SolverEighteen{}
    solvers[19] = solver.SolverNineteen{}
    solvers[20] = solver.SolverTwenty{}
    solvers[21] = solver.SolverTwentyOne{}
    solvers[22] = solver.SolverTwentyTwo{}
    solvers[23] = solver.SolverTwentyThree{}
    solvers[24] = solver.SolverTwentyFour{}
    solvers[25] = solver.SolverTwentyFive{}
}

func loadFileContent(filePath string) []string {
    file, err := os.Open(filePath)

    if err != nil {
        panic(err.Error())
    }
    defer file.Close()

    scanner := bufio.NewScanner(file)
    fileLines := []string{}

    for scanner.Scan() {
        fileLines = append(fileLines, scanner.Text())
    }

    return fileLines
}
