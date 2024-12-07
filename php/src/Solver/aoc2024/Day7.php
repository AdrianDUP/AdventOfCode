<?php

namespace Adriandup\Aoc\Solver\aoc2024;

use Adriandup\Aoc\Solver\Solver;

class Day7 extends Solver
{
    public function solutionOne(array $lines): int
    {
        $grid = $this->getGrid($lines);
        $start = $this->getStart($grid);

        $x = $start['x'];
        $y = $start['y'];

        $outOfBounds = false;

        $direction = "up";

        $xLimit = count($grid[0]);
        $yLimit = count($grid);

        /** @var array<string> $positionsVisited */
        $positionsVisited = [];

        do {
            $nextX = $this->nextX($x, $direction);
            $nextY = $this->nextY($y, $direction);

            if ($nextX = $xLimit || $nextY == $yLimit || $nextX < 0 || $nextY < 0) {
                $outOfBounds = true;
                continue;
            }

            $nextElement = $grid[$nextY][$nextX];

            if ($nextElement === '#') {
                $direction = $this->turn($direction);
            } else {
                $x = $nextX;
                $y = $nextY;

                $position = sprintf("%d,%d", $x, $y);

                if (!in_array($position, $positionsVisited)) {
                    $positionsVisited[] = $position;
                }
            }
        } while (!$outOfBounds);

        return count($positionsVisited);
    }

    public function solutionTwo(array $lines): int
    {
    }

    /**
     * @param array<string[]> $grid
     * @return array{
     *   x: int|null,
     *   y: int|null
     * }
     */
    private function getStart(array $grid): array {
        $start = [
            'x' => null,
            'y' => null,
        ];

        foreach ($grid as $rowIndex => $row) {
            foreach ($row as $columnIndex => $column) {
                if ($column === "^") {
                    $start['x'] = $columnIndex;
                    $start['y'] = $rowIndex;
                    return $start;
                }
            }
        }

        return $start;
    }

    private function turn(string $direction): string {
        return match ($direction) {
            "up" => "right",
            "right" => "down",
            "down" => "left",
            "left" => "up",
            default => $direction,
        };
    }

    private function nextX(int $x, string $direction): int {
        return match($direction) {
            "left" => --$x,
            "right" => ++$x,
            default => $x,
        };
    }

    private function nextY(int $y, string $direction): int {
        return match($direction) {
            "up" => --$y,
            "down" => ++$y,
            default => $y,
        };
    }
}
