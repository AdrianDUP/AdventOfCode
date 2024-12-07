<?php

namespace Adriandup\Aoc\Solver;

class SolverCollection
{
    /**
     * @var array<int, Solver> $solver
     */
    private array $solvers;

    public function __construct() {
    }

    public function add(Solver $solver, int $day): self {
        $this->solvers[$day] = $solver;
        return $this;
    }
}
