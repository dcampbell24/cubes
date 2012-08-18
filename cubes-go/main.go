// Copyright 2012 David Campbell.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

package main

import (
	"fmt"
	"os"
	"encoding/json"
)

// Read pieces from input file
// write OBJ files for each piece
// try to put the pieces together
// if successful, write an object file for the solution.
func main() {
	psJson, err := os.Open(os.Args[1])
	if err != nil {
		panic(err)
	}
	dec := json.NewDecoder(psJson)
	var ps []VecSlice
	if err := dec.Decode(&ps); err != nil {
		panic(err)
	}

	mtl, err := os.Create(fmt.Sprintf("polycubes.mtl"))
	if err != nil {
		panic(err)
	}
	for i, v := range ps {
		c := 1.0/float64(len(ps)) * float64(i)
		fmt.Fprintf(mtl, "newmtl %d\n", i)
		fmt.Fprintf(mtl, "Kd %f %f %f\n", c, c, c)
		fd, err := os.Create(fmt.Sprintf("polycube%d.obj", v[0].ID))
		if err != nil {
			panic(err)
		}
		fmt.Fprint(fd, "mtllib polycubes.mtl\n")
		fmt.Fprintf(fd, "usemtl %d\n", i)
		fmt.Fprint(fd, v.Trans(4*i-8, 0).Obj())
		fd.Close()
	}
	cube := NewCube(3)
	Search(ps, cube)
	for i, v := range SOL[0].VecSlices() {
		fd, err := os.Create(fmt.Sprintf("sol%d.obj", v[0].ID))
		if err != nil {
			panic(err)
		}
		fmt.Fprint(fd, "mtllib polycubes.mtl\n")
		fmt.Fprintf(fd, "usemtl %d\n", i)
		fmt.Fprint(fd, v.Trans(4, 2).Obj())
		fd.Close()
	}
	fmt.Println(SOL)
}
