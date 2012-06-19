// Copyright 2012 David Campbell.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

package cubes

import (
	"testing"
)

func TestEq(t *testing.T) {
	v1 := &Vec3{4, 8, 2, 1}
	v2 := &Vec3{4, 8, 2, 1}
	if !v1.Eq(v2) {
		t.Fail()
	}
}

func TestSub(t *testing.T) {
	v1 := &Vec3{5, 6, 7, 0}
	v2 := &Vec3{4, 8, 2, 0}
	v1.Sub(v1, v2)
	if !v1.Eq(&Vec3{1, -2, 5, 0}) {
		t.Fail()
	}
}
