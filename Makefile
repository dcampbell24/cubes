all: go-minotaur fortran-minotaur

go-minotaur: target/cubes-go
	target/cubes-go cubes-go/minotaur.json

fortran-minotaur: target/cubes-fortran
	target/cubes-fortran < puzzles/minotaur.txt

target/cubes-go:
	go build -o target/cubes-go \
	cubes-go/cubes.go \
	cubes-go/doc.go \
	cubes-go/cubes_test.go \
	cubes-go/main.go \
	cubes-go/sincos.go \
	cubes-go/vec3.go \
	cubes-go/vec3_test.go

target/cubes-fortran:
	gfortran -o target/cubes-fortran  fortran/cubes.f90 

clean:
	rm -f target/cubes-fortran target/cubes-go