package solver2024

import "aoc/pkg/solver"

func MakeSolverCollection() solver.SolverCollection {
    collection := make(map[int]solver.Solver)

    collection[1] = SolverOne{}
    collection[2] = SolverTwo{}
    collection[3] = SolverThree{}
    collection[4] = SolverFour{}
    collection[5] = SolverFive{}
    collection[6] = SolverSix{}
    collection[7] = SolverSeven{}
    collection[8] = SolverEight{}
    collection[9] = SolverNine{}
    collection[10] = SolverTen{}
    collection[11] = SolverEleven{}
    collection[12] = SolverTwelve{}
    collection[13] = SolverThirteen{}
    collection[14] = SolverFourteen{}
    collection[15] = SolverFifteen{}
    collection[16] = SolverSixteen{}
    collection[17] = SolverSeventeen{}
    collection[18] = SolverEighteen{}
    collection[19] = SolverNineteen{}
    collection[20] = SolverTwenty{}
    collection[21] = SolverTwentyOne{}
    collection[22] = SolverTwentyTwo{}
    collection[23] = SolverTwentyThree{}
    collection[24] = SolverTwentyFour{}
    collection[25] = SolverTwentyFive{}

    return solver.SolverCollection{
        Solvers: collection,
    }
}
