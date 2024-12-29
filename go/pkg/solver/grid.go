package solver

type Point struct {
    x: int64
    y: int64
}

type Grid struct {
    Grid: [][]string
    CurrentPosition: Point
}

func (grid *Grid) SetCurrentPosition(coordinate Point) {
    grid.CurrentPosition = coordinate;
}
