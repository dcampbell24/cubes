# Cubes

[![github]][github-link] [![REUSE status]][reuse]

[github]: https://img.shields.io/badge/github-8da0cb?logo=github
[github-link]: https://github.com/dcampbell24/mdbook-djot
[REUSE status]: https://api.reuse.software/badge/github.com/dcampbell24/cubes
[reuse]: https://api.reuse.software/info/github.com/dcampbell24/cubes

Applications for determining how to put a cube together and for creating
visualizations of the solution. There is a solver and an input program.
The solver reads the puzzles in RON format which the input program saves
the pieces as. The solve program writes the pieces and one of the solutions
as OBJ files. All data is written to the local data directory for the
application.

```sh
# On Linux
$HOME/.local/share/cubes # or
$XDG_DATA_HOME/cubes 
# On Mac
$HOME/Library/Application Support/Cubes
# On Windows
{FOLDERID_RoamingAppData}\Cubes\data
```

![Minotaur Cube](https://raw.githubusercontent.com/dcampbell24/cubes/master/cubes-rs/g3dviewer-solution.obj.png "One of two minotaur cube solutions")
