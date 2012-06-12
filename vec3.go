package cubes

import (
	"fmt"
)

type Vec3 struct {
	X, Y, Z int
	id int // the id of the piece the vector belongs to.
}

func (a *Vec3) Sub(b, c *Vec3) *Vec3 {
	a.X = b.X - c.X
	a.Y = b.Y - c.Y
	a.Z = b.Z - c.Z
	return a
}

func (a *Vec3) Mul(m [][]int, b *Vec3) *Vec3 {
	a.X = m[0][0]*b.X + m[0][1]*b.Y + m[0][2]*b.Z
	a.Y = m[1][0]*b.X + m[1][1]*b.Y + m[1][2]*b.Z
	a.Z = m[2][0]*b.X + m[2][1]*b.Y + m[2][2]*b.Z
	return a
}

func (a *Vec3) Cpy(b *Vec3) *Vec3 {
	a.X = b.X
	a.Y = b.Y
	a.Z = b.Z
	a.id = b.id
	return a
}

func (v *Vec3) String() string {
	return fmt.Sprintf("%d %d %d\n", v.X, v.Y, v.Z)
}

func (a *Vec3) Eq(b *Vec3) bool {
	return a.X == b.X &&
	       a.Y == b.Y &&
		   a.Z == b.Z &&
		   a.id == b.id
}
