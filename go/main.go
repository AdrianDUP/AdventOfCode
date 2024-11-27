package main

import (
	solver2024 "aoc/pkg/2024"
	"aoc/pkg/solver"
	"bufio"
	"fmt"
	"os"
	"strconv"
)

var (
solvers map[int]solver.SolverCollection
    day int
    part int
    filePath string
    year int
    errVar error
)

func main() {
    if len(os.Args) != 5 {
        panic("missing arguments")
    }

    day, errVar := strconv.Atoi(os.Args[2])

    if errVar != nil {
        panic("Invalid day")
    }

    part, errVar := strconv.Atoi(os.Args[3])

    if errVar != nil {
        panic("Invalid part number")
    }

    filePath := os.Args[4]

    year, errVar = strconv.Atoi(os.Args[1])

    if errVar != nil {
        panic("Invalid year number")
    }

    lines := loadFileContent(filePath)
    initSolvers()

    solv, ok := solvers[year]

    if !ok {
        panic("No solver collection for year")
    }

    answer := solv.Solve(day, part, lines)

    fmt.Printf("Answer for %d day %d part %d is %d", year, day, part, answer)
}

func initSolvers() {
    solvers = make(map[int]solver.SolverCollection)
    solvers[2024] = solver2024.MakeSolverCollection()
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
