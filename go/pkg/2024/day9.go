package solver2024

import (
    "unicode"
)

type SolverNine struct {
}

// parseDisk builds the block array and file info from the input line.
// Returns blocks (ids per position, -1 for free), fileStarts (by file id), fileLens (by file id)
func parseDisk(line string) ([]int, []int, []int) {
    // line is sequence of digits; even indices are file lengths, odd are free lengths
    // file id increments each file entry (even positions)
    // Compute total length first and max file id
    runLens := make([]int, 0, len(line))
    for _, ch := range line {
        if unicode.IsDigit(ch) {
            runLens = append(runLens, int(ch-'0'))
        }
    }
    total := 0
    fileCount := 0
    for i, v := range runLens {
        total += v
        if i%2 == 0 {
            fileCount++
        }
    }
    blocks := make([]int, total)
    fileStarts := make([]int, fileCount)
    fileLens := make([]int, fileCount)

    pos := 0
    fileID := 0
    for i, v := range runLens {
        if v == 0 {
            continue
        }
        if i%2 == 0 {
            // file
            fileStarts[fileID] = pos
            fileLens[fileID] = v
            for k := 0; k < v; k++ {
                blocks[pos+k] = fileID
            }
            pos += v
            fileID++
        } else {
            // free
            for k := 0; k < v; k++ {
                blocks[pos+k] = -1
            }
            pos += v
        }
    }

    return blocks, fileStarts, fileLens
}

// freeSeg represents a contiguous range of free blocks [start, start+length)
type freeSeg struct {
    start  int
    length int
}

// build initial free segments from blocks
func buildFreeSegments(blocks []int) []freeSeg {
    segs := make([]freeSeg, 0)
    n := len(blocks)
    i := 0
    for i < n {
        if blocks[i] == -1 {
            j := i + 1
            for j < n && blocks[j] == -1 {
                j++
            }
            segs = append(segs, freeSeg{start: i, length: j - i})
            i = j
        } else {
            i++
        }
    }
    return segs
}

// find leftmost free segment that starts before limit and has length >= need
// returns index in segs slice or -1 if none
func findLeftmostFit(segs []freeSeg, limit int, need int) int {
    for idx, s := range segs {
        if s.start >= limit {
            break
        }
        if s.length >= need {
            return idx
        }
    }
    return -1
}

// insert a free segment [start,len] into segs (sorted by start), merging neighbors if adjacent
func insertFreeSegment(segs []freeSeg, start int, length int) []freeSeg {
    // find insertion point
    i := 0
    for i < len(segs) && segs[i].start < start {
        i++
    }
    // insert at i
    segs = append(segs, freeSeg{})
    copy(segs[i+1:], segs[i:])
    segs[i] = freeSeg{start: start, length: length}
    // merge with previous
    if i-1 >= 0 {
        prev := segs[i-1]
        curr := segs[i]
        if prev.start+prev.length == curr.start {
            // merge prev and curr into prev
            segs[i-1].length += curr.length
            // remove curr at i
            copy(segs[i:], segs[i+1:])
            segs = segs[:len(segs)-1]
            i--
        }
    }
    // merge with next
    if i+1 < len(segs) {
        curr := segs[i]
        next := segs[i+1]
        if curr.start+curr.length == next.start {
            segs[i].length += next.length
            copy(segs[i+1:], segs[i+2:])
            segs = segs[:len(segs)-1]
        }
    }
    return segs
}

func (solver SolverNine) SolutionOne(lines []string) int {
    // Not required for this task; leave as-is.
    return 0
}

func (solver SolverNine) SolutionTwo(lines []string) int {
    if len(lines) == 0 {
        return 0
    }
    line := lines[0]

    blocks, fileStarts, fileLens := parseDisk(line)
    freeSegs := buildFreeSegments(blocks)

    // move files in decreasing id order
    for fid := len(fileStarts) - 1; fid >= 0; fid-- {
        L := fileLens[fid]
        if L == 0 {
            continue
        }
        start := fileStarts[fid]
        // find leftmost fit strictly before current start
        idx := findLeftmostFit(freeSegs, start, L)
        if idx == -1 {
            continue // no move
        }
        seg := freeSegs[idx]
        // Place file at seg.start
        newStart := seg.start
        // fill target positions with fid
        for k := 0; k < L; k++ {
            blocks[newStart+k] = fid
        }
        // update or remove the used free segment prefix
        if seg.length == L {
            // remove segment at idx
            copy(freeSegs[idx:], freeSegs[idx+1:])
            freeSegs = freeSegs[:len(freeSegs)-1]
        } else {
            // shrink segment from the left
            freeSegs[idx].start += L
            freeSegs[idx].length -= L
        }
        // free the old file location
        for k := 0; k < L; k++ {
            blocks[start+k] = -1
        }
        // insert freed segment and possibly merge
        freeSegs = insertFreeSegment(freeSegs, start, L)
        // update file start
        fileStarts[fid] = newStart
    }

    // compute checksum
    checksum := 0
    for i, v := range blocks {
        if v >= 0 {
            checksum += i * v
        }
    }
    return checksum
}
