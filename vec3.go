package cubes

import (
	"fmt"
)

type Vec3 struct {
	X, Y, Z int
	ID int // the id of the piece the vector belongs to.
}

func (a *Vec3) Sub(b, c *Vec3) *Vec3 {
	a.X = b.X - c.X
	a.Y = b.Y - c.Y
	a.Z = b.Z - c.Z
	return a
}

func (a *Vec3) Mul(m *[3][3]int, b *Vec3) *Vec3 {
	a.X = m[0][0]*b.X + m[0][1]*b.Y + m[0][2]*b.Z
	a.Y = m[1][0]*b.X + m[1][1]*b.Y + m[1][2]*b.Z
	a.Z = m[2][0]*b.X + m[2][1]*b.Y + m[2][2]*b.Z
	return a
}

func (a *Vec3) Cpy(b *Vec3) *Vec3 {
	a.X = b.X
	a.Y = b.Y
	a.Z = b.Z
	a.ID = b.ID
	return a
}

func (v *Vec3) String() string {
	return fmt.Sprintf("%d %d %d\n", v.X, v.Y, v.Z)
}

func (a *Vec3) Eq(b *Vec3) bool {
	return a.X == b.X &&
	       a.Y == b.Y &&
		   a.Z == b.Z &&
		   a.ID == b.ID
}

func (v Vec3) cube() *OBJ {
	return &OBJ{[][3]int{{v.X,   v.Y,   v.Z},    // 1
	                     {v.X+1, v.Y,   v.Z},    // 2
	                     {v.X,   v.Y+1, v.Z},    // 3
	                     {v.X+1, v.Y+1, v.Z},    // 4
	                     {v.X,   v.Y,   v.Z+1},  // 5
	                     {v.X+1, v.Y,   v.Z+1},  // 6
	                     {v.X,   v.Y+1, v.Z+1},  // 7
	                     {v.X+1, v.Y+1, v.Z+1}}, // 8
	           [][4]int {{1, 2, 4, 3},
	                     {1, 5, 7, 3},
	                     {1, 2, 6, 5},
	                     {8, 7, 3, 4},
	                     {8, 7, 5, 6},
	                     {8, 4, 2, 6}}}
}
