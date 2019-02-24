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
        .workdir("/app")
        .comment("te")
        .copy_from("build", "/src/goapp", "/app/")
        .entrypoint(&["./goapp"]);

    let file = DockerFile::new()
             .dockerfile(build)
             .dockerfile(run);

    println!("{}", file.to_string());
}