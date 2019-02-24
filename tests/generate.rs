use docker_generate::DockerFile;

#[test]
fn geterate() {

    let build = 
        DockerFile::new()
        .comment("build stage")
        .from_alias("build", "golang:alpine")
        .run("cd /src && go build -o goapp");

    let run = 
        DockerFile::new()
        .comment("run stage")
        .from("alpine")
        .run("cargo install --path .")
        .cmd(&["myapp"]);

    let file = DockerFile::new()
             .dockerfile(build)
             .newline()
             .dockerfile(run);

    println!("{}", file.to_string());
}