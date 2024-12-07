<?php

namespace Adriandup\Aoc\Solver;

abstract class Solver
{
    /**
     * @param string[] $lines
     */
    abstract public function solutionOne(array $lines): int;
    /**
     * @param string[] $lines
     */
    abstract public function solutionTwo(array $lines): int;

    /**
     * @return string[]
     */
    protected function splitStringIntoCharacters(string $line): array {
        $characters = explode("", $line);

        if (is_array($characters)) {
            return $characters;
        }

        return [];
    }

    /**
     * @param string[] $lines
     * @return array<string[]>
     */
    protected function getGrid(array $lines): array {
        $rows = [];

        foreach ($lines as $line) {
            $rows[] = $this->splitStringIntoCharacters($line);
        }

        return $rows;
    }
}
