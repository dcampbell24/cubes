// Copyright 2012 David Campbell.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

package main

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
