go-minotaur: target/cubes-go
	target/cubes-go cubes-go/minotaur.json

target/cubes-go:
	go build -o target/cubes-go \
	cubes-go/cubes.go \
	cubes-go/doc.go \
	cubes-go/cubes_test.go \
	cubes-go/main.go \
	cubes-go/sincos.go \
	cubes-go/vec3.go \
	cubes-go/vec3_test.go
