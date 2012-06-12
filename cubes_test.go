package cubes

import (
	"fmt"
	"testing"
)

func TestAllRots(_ *testing.T) {
	r := slugs[0].AllRots()
	for _, v := range r {
		fmt.Println(v)
	}
}

func TestSearch(_ *testing.T) {
	cube := newCube(3)
	//search(minotaur, cube)
	search(slugs, cube)
	fmt.Println(SOL)
	fmt.Println(len(SOL))
}

func BenchmarkSearch(b *testing.B) {
	for i := 0; i < b.N; i++ {
		cube := newCube(3)
		search(slugs, cube)
	}
}
