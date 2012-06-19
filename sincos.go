// Copyright 2012 David Campbell.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

package cubes

func sin(theta int) int {
	switch theta {
	case 0:
		return 0
	case 90:
		return 1
	case 180:
		return 0
	case 270:
		return -1
	}
	panic("invalid angle")
}

func cos(theta int) int {
	switch theta {
	case 0:
		return 1
	case 90:
		return 0
	case 180:
		return -1
	case 270:
		return 0
	}
	panic("invalid angle")
}
