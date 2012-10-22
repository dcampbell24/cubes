// Copyright 2012 David Campbell.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

package main

import (
	"fmt"
	"testing"
)

var (
	minotaur = []VecSlice{{{0,1,0,2}, {1,1,0,2}, {1,0,0,2}, {1,0,1,2}},
						  {{0,1,0,3}, {1,1,0,3}, {1,1,1,3}, {1,0,1,3}},
						  {{0,0,0,5}, {1,0,0,5}, {2,0,0,5}, {1,1,0,5}},
						  {{0,0,1,4}, {0,1,1,4}, {0,1,0,4}, {0,2,0,4}, {1,1,0,4}},
		                  {{0,2,0,6}, {1,0,1,6}, {1,1,1,6}, {1,1,0,6}, {1,2,0,6}},
						  {{0,0,0,1}, {0,1,0,1}, {0,2,0,1}, {1,0,0,1}, {0,1,1,1}}}

	slugs    = []VecSlice{{{0,0,0,1}, {1,0,0,1}, {2,0,0,1},
	                       {0,1,0,1}, {1,1,0,1}, {2,1,0,1},
	                       {0,2,0,1}, {1,2,0,1}, {2,2,0,1}},

	                      {{0,0,0,2}, {1,0,0,2}, {2,0,0,2},
	                       {0,1,0,2}, {1,1,0,2}, {2,1,0,2},
	                       {0,2,0,2}, {1,2,0,2}, {2,2,0,2}},

		                  {{0,0,0,3}, {1,0,0,3}, {2,0,0,3},
	                       {0,1,0,3}, {1,1,0,3}, {2,1,0,3},
	                       {0,2,0,3}, {1,2,0,3}, {2,2,0,3}}}
)

func TestSearch(_ *testing.T) {
	cube := NewCube(3)
	Search(minotaur, cube)
	Search(slugs, cube)
	fmt.Println(SOL)
	fmt.Println(len(SOL))
}