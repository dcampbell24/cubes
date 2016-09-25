Puzzles
========

This directory contains data files in a format readable by the Fortran version
of the cubes solver. All of the data is from other sources than the author of
this code, but he needs to do some research to determine what they are.

```sh
# These data files are a little peculiar due to the nature of Fortran's built in
# I/O functionality.

# The name followed by an ordered listing of the pieces. The next line says
# the number of pieces so that the size of the array needed to hold all of the
# pieces is known ahead of starting to parse the data for each piece.  
MINOTAUR 2, 3, 5, 4, 6, 1
6

# This piece is made up of four unit cubes, each described by a 3 dimensional
# column vector. The first `1` indicates this piece is unique.
4 1
1 2 2 2 # x component
2 2 1 1 # y component
1 1 1 2 # z component

4 1
1 2 2 2
2 2 2 1
1 1 2 2

4 1
1 2 3 2
1 1 1 2
1 1 1 1

5 1
1 1 1 1 2
1 2 2 3 2
2 2 1 1 1

5 1
1 2 2 2 2
3 1 2 2 3
1 2 2 1 1

5 1
1 1 1 2 1
1 2 3 1 2
1 1 1 1 2
```
