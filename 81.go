package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

type matrix [][]uint16

func load() matrix {
	f, err := os.Open("p081_matrix.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()

	m := make([][]uint16, 80)
	for i := range m {
		m[i] = make([]uint16, 80)
	}

	scanner := bufio.NewScanner(f)
	for i := range m {
		if !scanner.Scan() {
			if err := scanner.Err(); err != nil {
				panic(err)
			}
			panic("too short")
		}

		line := scanner.Text()
		numberStrings := strings.Split(line, ",")
		for j := range m[i] {
			n64, err := strconv.ParseUint(numberStrings[j], 10, 16)
			if err != nil {
				panic(err)
			}
			m[i][j] = uint16(n64)
		}
	}

	if scanner.Scan() {
		panic("too long")
	}
	if err := scanner.Err(); err != nil {
		panic(err)
	}

	return m
}

func (m matrix) bruteForce(start, currentSum int) int {
	if len(m) == 1 {
		for i := start; i < len(m[0]); i++ {
			currentSum += int(m[0][i])
		}
		return currentSum
	}

	minSum := math.MaxInt
	thisSum := currentSum
	for i := start; i < len(m[0]); i++ {
		thisSum += int(m[0][i])

		foundSum := m[1:].bruteForce(i, thisSum)
		if foundSum < minSum {
			minSum = foundSum
		}
	}
	return minSum
}

func main() {
	m := matrix{
		{131, 673, 234, 103, 18},
		{201, 96, 342, 965, 150},
		{630, 803, 746, 422, 111},
		{537, 699, 497, 121, 956},
		{805, 732, 524, 37, 331},
	}

	// m := load()

	fmt.Println(m.bruteForce(0, 0))
}
