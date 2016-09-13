#[macro_use]
extern crate cake;

build! {
    go_minotaur(compile_go_minotaur) => cmd!("target/cubes-go", "cubes-go/minotaur.json"),

    compile_go_minotaur() => cmd!(
        "go", "build", "-o", "target/cubes-go",
        "cubes-go/cubes.go",
        "cubes-go/doc.go",
        "cubes-go/cubes_test.go",
        "cubes-go/main.go",
        "cubes-go/sincos.go",
        "cubes-go/vec3.go",
        "cubes-go/vec3_test.go"
    ),
}
