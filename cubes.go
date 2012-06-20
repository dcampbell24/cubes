// Copyright 2012 David Campbell.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

// Cubes is a package for manipulating the pieces of a cube in order to find ways of
// putting them together.
package cubes

import (
	"fmt"
	"sort"
)

const (
	X = iota
	Y
	Z
)

var (
	Minotaur = []VecSlice{{{0,1,0,2}, {1,1,0,2}, {1,0,0,2}, {1,0,1,2}},
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

// X, Y, Z rotation matricies
var mrots [4][3][3][3]int

func init() {
	for i := range mrots {
	θ := i*90
	mrot := [3][3][3]int{{{      1,       0,       0},
	                      {      0,  cos(θ), -sin(θ)},
	                      {      0,  sin(θ),  cos(θ)}},
	                     {{ cos(θ),       0,  sin(θ)},
	                      {      0,       1,       0},
	                      {-sin(θ),       0,  cos(θ)}},
	                     {{ cos(θ), -sin(θ),       0},
	                      { sin(θ),  cos(θ),       0},
	                      {      0,       0,       1}}}
	mrots[i] = mrot
	}
}

type OBJ struct {
	//name string
	vs [][3]int // verticies
	fs [][4]int // faces
}

func (o OBJ) String() string {
	str := "# Go generated OBJ file.\n"
	for _, v := range o.vs {
		str += fmt.Sprintf("v %d %d %d\n", v[0], v[1], v[2])
	}
	for _, f := range o.fs {
		str += fmt.Sprintf("f %d %d %d %d\n", f[0], f[1], f[2], f[3])
	}
	return str
}

// This could be represented with fewer verticies and faces, but would be more
// work and may not be of much benefit.
func (a *OBJ) Join(b *OBJ) {
	l := len(a.vs)
	a.vs = append(a.vs, b.vs...)
	for i, f := range b.fs {
		for j := range f {
			b.fs[i][j] += l
		}
	}
	a.fs = append(a.fs, b.fs...)
}

type VecSlice []*Vec3

func NewVecSlice(i int) VecSlice {
	a := make(VecSlice, i)
	for i := range a {
		a[i] = new(Vec3)
	}
	return a
}

func (s VecSlice) Obj() *OBJ {
	obj := new(OBJ)
	for _, v := range s {
		obj.Join(v.cube())
	}
	return obj
}


func (cube Cube) VecSlice() VecSlice {
	s := VecSlice{}
	for i, x := range cube {
		for j, y := range x {
			for k, z := range y {
				if z != 0 {
					s = append(s, &Vec3{X:i, Y:j, Z:k, ID: 1})
				} else {
					s = append(s, &Vec3{X:i, Y:j, Z:k, ID: 0})
				}
			}
		}
	}
	return s
}

func (cube Cube) VecSlices() []VecSlice {
	p := 0
	for _, x := range cube {
		for _, y := range x {
			for _, z := range y {
				if z > p {
					p = z
				}
			}
		}
	}
	s := make([]VecSlice, p)
	for i, x := range cube {
		for j, y := range x {
			for k, z := range y {
				s[z-1] = append(s[z-1], &Vec3{X:i, Y:j, Z:k, ID: z})
			}
		}
	}
	return s
}

func (s VecSlice) String() string {
	str := ""
	for _, v := range s {
		str += v.String()
	}
	return str
}

// Translate the VecSlice so that it's min x, y, and z values are all 0 and
// sort the vectors.
func (a VecSlice) Canonical(b VecSlice) VecSlice {
	a.Cpy(b)
	sort.Sort(a)
	a.PushToZero()
	return a
}

func (a VecSlice) IsLegal(cube Cube) bool {
	for _, v := range a {
		if cube[v.X][v.Y][v.Z] != 0 {
			return false
		}
	}
	return true
}

func (a VecSlice) PushToZero() {
	x, y, z := a[0].X, a[0].Y, a[0].Z
	for _, v := range a {
		if v.X < x {
			x = v.X
		}
		if v.Y < y {
			y = v.Y
		}
		if v.Z < z {
			z = v.Z
		}
	}
	a.Trans(-x, X)
	a.Trans(-y, Y)
	a.Trans(-z, Z)
}

func (a VecSlice) Max() (int, int, int) {
	x, y, z := a[0].X, a[0].Y, a[0].Z
	for _, v := range a {
		if v.X > x {
			x = v.X
		}
		if v.Y > y {
			y = v.Y
		}
		if v.Z > z {
			z = v.Z
		}
	}
	return x, y, z
}

// Start by pushing a piece into a corner, then push it along each axis until
// it goes out of bounds in that direction, keeping track of all of the
// placements which are legal.
//
// How to push into a corner... find the min x y and z values from all of the
// vectors and then translate all of the vectors to set all of those values to
// 0, next find the max x, y, and z values and push the piece till those
// go out of bounds.
func (sl VecSlice) AllPuts(cube Cube) []VecSlice {
	allputs := make([]VecSlice, 0)
	for _, s := range sl.AllRots() {
		Xmax, Ymax, Zmax := s.Max()
		for x := Xmax; x < 3; x++ {
			px := NewVecSlice(len(s))
			px.Cpy(s)
			px.Trans(x - Xmax, X)
			for y := Ymax; y < 3; y++ {
				py := NewVecSlice(len(px))
				py.Cpy(px)
				py.Trans(y - Ymax, Y)
				for z := Zmax; z < 3; z++ {
					pz := NewVecSlice(len(py))
					pz.Cpy(py)
					pz.Trans(z - Zmax, Z)
					if pz.IsLegal(cube) {
						allputs = append(allputs, pz)
					}
				}
			}
		}
	}
	return allputs
}

func (s VecSlice) Len() int {
	return len(s)
}

func (s VecSlice) Swap(i, j int) {
	s[i], s[j] = s[j], s[i]
}

func (s VecSlice) Less(i, j int) bool {
	if s[i].X < s[j].X {
		return true
	} else if s[i].X > s[j].X {
		return false
	} else if s[i].Y < s[j].Y {
		return true
	} else if s[i].Y > s[j].Y {
		return false
	} else if s[i].Z < s[j].Z {
		return true
	}
	return false
}

// Assumes the VecSlices are already in canonical form.
func (a VecSlice) Eq(b VecSlice) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if !a[i].Eq(b[i]) {
			return false
		}
	}
	return true
}

func (a VecSlice) Cpy(b VecSlice) VecSlice {
	for i, v := range a {
		v.Cpy(b[i])
	}
	return a
}

func (s VecSlice) Rot(θ, axis int) VecSlice {
	v2 := new(Vec3)
	for _, v := range s {
		v2.Cpy(v)
		v.Mul(&mrots[θ/90][axis], v2)
	}
	return s
}

// Slow linear search!
func lsearch(elem VecSlice, coll []VecSlice) bool {
	for _, v := range coll {
		if elem.Eq(v) {
			return true
		}
	}
	return false
}

func (s VecSlice) AllRots() []VecSlice {
	s1 := NewVecSlice(len(s))
	s1.Cpy(s)
	canon := NewVecSlice(len(s))
	rots := make([]VecSlice, 0)
	for i := 0; i < 4; i++ {
		for j := 0; j < 4; j++ {
			for k := 0; k < 4; k++ {
				canon.Canonical(s1)
				if !lsearch(canon, rots) {
					r := NewVecSlice(len(canon))
					r.Cpy(canon)
					rots = append(rots, r)
				}
				s1.Rot(90, Z)
			}
			s1.Rot(90, Y)
		}
		s1.Rot(90, X)
	}
	return rots
}

func (vs VecSlice) Trans(dist, axis int) VecSlice {
	switch axis {
	case X:
		for _, v := range vs {
			v.X += dist
		}
	case Y:
		for _, v := range vs {
			v.Y += dist
		}
	case Z:
		for _, v := range vs {
			v.Z += dist
		}
	}
	return vs
}

type Cube [][][]int

func (cube Cube) String() string {
	s := ""
	for _, x := range cube {
		s += "\n"
		for _, y := range x {
			s += "\n"
			for _, z := range y {
				s += fmt.Sprintf("%v ", z)
			}
		}
	}
	return s + "\n------------------------"
}

func NewCube(n int) Cube {
	cube := make([][][]int, n)
	for i := range cube {
		cube[i] = make([][]int, n)
		for j := range cube[i] {
			cube[i][j] = make([]int, n)
		}
	}
	return cube
}

func (a Cube) Cpy(b Cube) Cube {
	for i, w := range b {
		for j, v := range w {
			copy(a[i][j], v)
		}
	}
	return a
}

func (c Cube) Place(s VecSlice, n int) {
	for _, v := range s {
		c[v.X][v.Y][v.Z] = n
	}
}

var SOL = make([]Cube, 0)

func Search(ss []VecSlice, cube Cube) {
	if len(ss) == 0 {
		SOL = append(SOL, cube)
		return
	}
	puts := make([]Cube, 0)
	cputs := make([]VecSlice, 0)
loop:
	for _, v := range ss[len(ss)-1].AllPuts(cube) {
		c1 := NewCube(3)
		c1.Cpy(cube)
		c1.Place(v, v[0].ID)
		allRots := c1.VecSlice().AllRots()
		for _, w := range cputs {
			for _, rot := range allRots {
				if w.Eq(rot) {
					continue loop
				}
			}
		}
		puts = append(puts, c1)
		canon := c1.VecSlice()
		cputs = append(cputs, canon.Canonical(canon))
	}
	for _, p := range puts {
		Search(ss[:len(ss)-1], p)
	}
}
