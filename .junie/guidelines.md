# AdventOfCode – Project Guidelines and Overview

This repository contains multi-language solutions (Rust, Go, PHP) for Advent of Code across multiple years. It is organized to let you run individual days/parts easily and keep puzzle inputs under versioned folders.

## Repository layout (high level)
- 2015/, 2023/, 2024/: Year-specific directories for Go solutions (wrappers around shared Go code)
- go/
  - pkg/2015, pkg/2023, pkg/2024: Go solvers grouped by year
  - pkg/solver: Common Go interfaces/utilities
  - inputs/2023, inputs/2024: Go inputs (dayN.txt, dayN_test.txt)
  - main.go: Go CLI entrypoint
- rust/aoc/
  - src/: Rust CLI and solver implementations
    - src/solver/s23, src/solver/s24: Year-specific Rust solvers
  - inputs/2023, inputs/2024: Rust inputs (dayN.txt, dayN_test.txt)
  - Cargo.toml/Cargo.lock (implicit): Rust crate metadata
- php/src/: PHP helpers and solvers (AoC 2024)
- README.md: General repo notes
- .junie/: Automation and this guidelines file

## Inputs
- Rust expects inputs at:
  - rust/aoc/inputs/{YEAR}/day{N}.txt
  - rust/aoc/inputs/{YEAR}/day{N}_test.txt
  - When running from rust/aoc working directory, it also supports ./inputs/{YEAR}/...
- Go generally reads a file path you provide on the command line. Example inputs live under:
  - go/inputs/{YEAR}/day{N}.txt
  - go/inputs/{YEAR}/day{N}_test.txt

## How to run solutions

Rust (from repo root or from rust/aoc):
- cd to repo root OR rust/aoc
- Command format: cargo run -- <YEAR> <DAY> <PART> [test]
  - YEAR: 2023 or 2024 (as implemented)
  - DAY: e.g., 10
  - PART: 1 or 2
  - Add the literal word test as 4th arg to use the _test.txt input
- Examples:
  - From repo root: cargo run -q --manifest-path rust/aoc/Cargo.toml -- 2024 10 1
  - From rust/aoc: cargo run -q -- 2024 10 2 test

Go (from repo root or go/):
- Command format: go run ./go/main.go <YEAR> <DAY> <PART> <INPUT_PATH>
- Examples:
  - go run ./go/main.go 2024 9 1 ./go/inputs/2024/day9.txt
  - go run ./go/main.go 2024 9 2 ./go/inputs/2024/day9_test.txt

PHP
- PHP code exists but there is no unified CLI documented in this repo; prefer Rust/Go runners above for now.

## Adding a new solution day
- Rust:
  - Create a new file under rust/aoc/src/solver/sYY/dayN.rs implementing the Solver trait (see existing days for reference)
  - Register the day in rust/aoc/src/main.rs within get_solvers() for the proper year
  - Add input files under rust/aoc/inputs/{YEAR}/
- Go:
  - Add a new solver implementation under go/pkg/{YEAR}/ and wire it into MakeSolverCollection()
  - Add input files under go/inputs/{YEAR}/

## Tests and verification
- There are no formal unit tests integrated at the moment. Use dayN_test.txt files to quickly verify logic against sample inputs.
- For PRs/automation, prefer to:
  - Run a relevant Rust command with and without the test flag
  - Run the Go command against both dayN.txt and dayN_test.txt

## Build and tooling notes for Junie
- Prefer minimal edits focused on the specific issue.
- If you need to run Rust from the repo root, pass --manifest-path rust/aoc/Cargo.toml to cargo commands.
- Do not introduce new external dependencies unless required by the issue.
- Keep code changes small and language idiomatic (rustfmt/go fmt conventions).

## Code style
- Rust: idiomatic Rust, format with rustfmt; avoid unnecessary allocations; prefer iterators where clear.
- Go: gofmt, clear error handling; small functions; keep interfaces minimal.
- PHP: PSR-12 style if editing.

## What Junie should do before submitting
- Ensure the requested change is implemented with minimal scope.
- If commands/paths are referenced in the change, validate them against the code (main.rs, main.go).
- When applicable, run or document example commands in the commit message/description to aid reviewers.
