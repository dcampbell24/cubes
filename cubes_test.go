// Copyright 2012 David Campbell.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

package main

import (
	"fmt"
	"testing"
)

/*
func TestAllRots(_ *testing.T) {
	r := slugs[0].AllRots()
	for _, v := range r {
		fmt.Println(v)
	}
}

func TestAllPuts(_ *testing.T) {
	r := slugs[0].AllPuts(newCube(3))
	for _, v := range r {
		fmt.Println(v)
	}
}
*/

func TestSearch(_ *testing.T) {
	cube := NewCube(3)
	Search(Minotaur, cube)
	Search(slugs, cube)
	fmt.Println(SOL)
	fmt.Println(len(SOL))
}
/*
func BenchmarkSearch(b *testing.B) {
	for i := 0; i < b.N; i++ {
		cube := newCube(3)
		search(slugs, cube)
	}
}
*/
