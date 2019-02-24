```
use docker_generate::DockerFile;

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
```

```
# build stage
FROM golang:alpine as build
RUN cd /src && go build -o goapp

# run stage
FROM alpine
WORKDIR /app
COPY --from=build /src/goapp /app/
ENTRYPOINT ["./goapp"]
```


OR
```
let build = 
    DockerFile::new()
    .add("#", "My app".into())
    .add("FROM", "alpine".into())
    .add("WORKDIR", "/app".into())
    .add("COPY", vec![".", "/app"].into())
    .add("CMD", DockerFieldType::Array(vec!["./app/main"]));
```

```
# My app
FROM alpine
WORKDIR /app
COPY . /app
CMD ["./app/main"]
```