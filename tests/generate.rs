use docker_generate::{DockerFile, DockerFieldType};

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
        .copy_from("build", "/src/goapp", "/app/")
        .entrypoint(&["./goapp"]);

    let file = DockerFile::new()
             .dockerfile(build)
             .dockerfile(run);

    println!("{}", file.to_string());

    let build_2 = 
        DockerFile::new()
        .add("#", "My app".into())
        .add("FROM", "alpine".into())
        .add("WORKDIR", "/app".into())
        .add("COPY", vec![".", "/app"].into())
        .add("CMD", DockerFieldType::Array(vec!["./app/main"]));

    println!("{}", build_2.to_string());
}