package main

import (
	"fmt"
	"math"
	"math/big"
)

type iter struct {
	min, max int
	v        []int
}

func newIter(length, min, max int) *iter {
	if length < 1 {
		panic("length < 1")
	}
	if min >= max {
		panic("min >= max")
	}
	if min == math.MinInt {
		panic("min == math.MinInt")
	}

	v := make([]int, length)
	for i := range v {
		v[i] = min
	}
	v[len(v)-1] = min - 1
	return &iter{
		min: min,
		max: max,
		v:   v,
	}
}

func (it *iter) next() bool {
	for i := len(it.v) - 1; i >= 0; i-- {
		if it.v[i] < it.max {
			it.v[i]++
			return true
		} else {
			it.v[i] = it.min
		}
	}
	return false
}

func (it *iter) borrowValue() []int {
	return it.v
}

func pow(base, exp int) int {
	if exp < 1 {
		panic("exp < 1")
	}

	n := base
	for i := 1; i < exp; i++ {
		n *= base
	}
	return n
}

func sum(s []int) int {
	sum := 0
	for _, v := range s {
		sum += v
	}
	return sum
}

func diceDistribution(dice, sides int) (int, []int) {
	count := 0
	distribution := make([]int, dice*sides+1)
	for it := newIter(dice, 1, sides); it.next(); count++ {
		distribution[sum(it.borrowValue())]++
	}
	countCalc := pow(sides, dice)
	if count != countCalc {
		panic("internal")
	}
	return countCalc, distribution
}

func main() {
	peterDenom, peter := diceDistribution(9, 4)
	colinDenom, colin := diceDistribution(6, 6)
	var all big.Rat
	for i, c := range colin {
		var peterAbove big.Rat
		for _, p := range peter[i+1:] {
			peterAbove.Add(&peterAbove, big.NewRat(int64(p), int64(peterDenom)))
		}
		peterAbove.Mul(&peterAbove, big.NewRat(int64(c), int64(colinDenom)))
		all.Add(&all, &peterAbove)
	}
	fmt.Println(all.FloatString(7))
}
