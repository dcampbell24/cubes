// Copyright 2012 David Campbell.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

package main

import (
	"fmt"
	"os"
)

// Read pieces from input file
// write OBJ files for each piece
// try to put the pieces together
// if successful, write an object file for the solution.
func main() {
	mtl, err := os.Create(fmt.Sprintf("minotaur.mtl"))
	if err != nil {
		panic(err)
	}
	for i, v := range Minotaur {
		c := 1.0/float64(len(Minotaur)) * float64(i)
		fmt.Fprintf(mtl, "newmtl %d\n", i)
		fmt.Fprintf(mtl, "Kd %f %f %f\n", c, c, c)
		fd, err := os.Create(fmt.Sprintf("minotaur%d.obj", v[0].ID))
		if err != nil {
			panic(err)
		}
		fmt.Fprint(fd, "mtllib minotaur.mtl\n")
		fmt.Fprintf(fd, "usemtl %d\n", i)
		fmt.Fprint(fd, v.Trans(4*i-8, 0).Obj())
		fd.Close()
	}
	cube := NewCube(3)
	Search(Minotaur, cube)
	for i, v := range SOL[0].VecSlices() {
		fd, err := os.Create(fmt.Sprintf("sol%d.obj", v[0].ID))
		if err != nil {
			panic(err)
		}
		fmt.Fprint(fd, "mtllib minotaur.mtl\n")
		fmt.Fprintf(fd, "usemtl %d\n", i)
		fmt.Fprint(fd, v.Trans(4, 2).Obj())
		fd.Close()
	}
	fmt.Println(SOL)
}
