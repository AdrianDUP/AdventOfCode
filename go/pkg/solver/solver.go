package solver

type Solver interface {
    SolutionOne([]string) int
    SolutionTwo([]string) int
}

type SolverCollection struct {
    Solvers map[int]Solver
}

func (collect SolverCollection) Solve(day int, part int, input []string) int {
    solver, ok := collect.Solvers[day]

    if !ok {
        panic("Solver not found for the given day")
    }
    
    switch part {
    case 1:
        return solver.SolutionOne(input)
    case 2:
        return solver.SolutionTwo(input)
    default:
        panic("Invalid part number given.")
    }
}
