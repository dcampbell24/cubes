Cubes
======

Applications for determining how to put a cube together and for creating
visualizations of the solution. The Rust version is superior to the
others.

![Minotaur Cube](https://github.com/davekong/cubes/raw/master/cube.png "One of two minotaur cube solutions")

Directories
-----------

* cubes-go: Go version of the solver. Reads puzzles in json form and can be
  downloaded with `go get github.com/davekong/cubes/cubes-go`

* cubes-rs: Rust version of the solver, a drawing program, and an input
  program. Reads the puzzles in bincode format which the input program
  saves the pieces as. Writes the pieces and one of the solutions as OBJ
  files. 

* fortran: Fortran version of the solver. Solves puzzles in the format of the
  files in the puzzles directory.

* julia: Julia version of the solver. Puzzles are stored using internal Julia
  arrays and the application must used in the REPL.

* puzzles: Puzzles to solve written in a plain text format.

* utils: A python script for importing solutions to blender, C code for
  rendering an interactive 3-D visualization of a solution, and a tcl script
  for creating a puzzle file.
