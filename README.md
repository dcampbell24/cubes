Cubes
======

Applications for determining how to put a cube together and for creating
visualizations of the solution.  Currently, only the Go version generates OBJ
files for use with the viewers; however, the Fortran version is around a
thousand times faster.

![Minotaur Cube](https://github.com/davekong/cubes/raw/master/cube.png "One of two minotaur cube solutions")

Directories
-----------

* cubes-go: Go version of the solver. Reads puzzles in json form and can be
  downloaded with `go get github.com/davekong/cubes/cubes-go`

* fortran: Fortran version of the solver. Solves puzzles in the format of the
  files in the puzzles directory.

* julia: Julia version of the solver. Puzzles are stored using internal Julia
  arrays and the application must used in the REPL.

* puzzles: Puzzles to solve written in a plain text format.

* utils: A python script for importing solutions to blender, C code for
  rendering an interactive 3-D visualization of a solution, and a tcl script
  for creating a puzzle file.
