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

type VecSlice []*Vec3

func NewVecSlice(i int) VecSlice {
	a := make(VecSlice, i)
	for i := range a {
		a[i] = new(Vec3)
	}
	return a
}

func (cube Cube) VecSlice() VecSlice {
	s := VecSlice{}
	for i, x := range cube {
		for j, y := range x {
			for k, z := range y {
				if z != 0 {
					s = append(s, &Vec3{X:i, Y:j, Z:k, id: 1})
				} else {
					s = append(s, &Vec3{X:i, Y:j, Z:k, id: 0})
				}
			}
		}
	}
	return s
}

// This does not do what I want.
func (a VecSlice) Congru(b VecSlice) bool {
	a1 := NewVecSlice(len(a)).Canonical(a)
	b1 := NewVecSlice(len(b))
	for _, sb := range b.AllRots() {
		if a1.Eq(b1.Canonical(sb)) {
			return true
		}
	}
	return false
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
	Xmin, Ymin, Zmin := a.Min()
	a.Trans(-Xmin, X)
	a.Trans(-Ymin, Y)
	a.Trans(-Zmin, Z)
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

// Return which vectors in the slice have the min x, y, and z values.
func (a VecSlice) Min() (int, int, int) {
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
	return x, y, z
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
		// FIXME redundant
		Xmin, Ymin, Zmin := s.Min()
		s.Trans(-Xmin, X)
		s.Trans(-Ymin, Y)
		s.Trans(-Zmin, Z)

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

func newCube(n int) Cube {
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

func search(ss []VecSlice, cube Cube) {
	if len(ss) == 0 {
		SOL = append(SOL, cube)
		return
	}
	puts := make([]Cube, 0)
loop:
	for _, v := range ss[len(ss)-1].AllPuts(cube) {
		c1 := newCube(3)
		c1.Cpy(cube)
		c1.Place(v, v[0].id)
		vs := c1.VecSlice()
		for _, w := range puts {
			if w.VecSlice().Congru(vs) {
				continue loop
			}
		}
		puts = append(puts, c1)
	}
	for _, p := range puts {
		search(ss[:len(ss)-1], p)
	}
}
