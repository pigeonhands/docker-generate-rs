
pub enum DockerFieldType<'a>{
    Docker(DockerFile<'a>),
    r#String(String),
    Array(&'a [&'a str]),
    TupleList(&'a[(&'a str, &'a str)]),
    MultiValue(&'a [&'a str]),
    Protocal((i32, &'a str)),
    Int(i32),
}

impl<'a> From<&'a [&'a str]> for DockerFieldType<'a>{
    fn from(value: &'a [&'a str]) -> Self {
        DockerFieldType::MultiValue(value)
    }
}


impl<'a> From<&'a str> for DockerFieldType<'a>{
    fn from(value: &'a str) -> Self {
        DockerFieldType::String(value.to_string())
    }
}

impl<'a> From<String> for DockerFieldType<'a>{
    fn from(value: String) -> Self {
        DockerFieldType::String(value)
    }
}

impl<'a> From<DockerFile<'a>> for DockerFieldType<'a>{
    fn from(value: DockerFile<'a>) -> Self {
        DockerFieldType::Docker(value)
    }
}

impl<'a> From<&'a[(&'a str, &'a str)]> for DockerFieldType<'a>{
    fn from(value: &'a[(&'a str, &'a str)]) -> Self {
        DockerFieldType::TupleList(value)
    }
}

impl<'a> From<(i32, &'a str)> for DockerFieldType<'a>{
    fn from(value: (i32, &'a str)) -> Self {
        DockerFieldType::Protocal(value)
    }
}

impl<'a> From<i32> for DockerFieldType<'a>{
    fn from(value: i32) -> Self {
        DockerFieldType::Int(value)
    }
}

impl <'a> DockerFieldType<'a>{
    pub fn to_string(&self) -> String{
        match self {
            DockerFieldType::Docker(d) => d.to_string(),
            DockerFieldType::String(s) => s.to_string(),
            DockerFieldType::Int(i) => i.to_string(),
            DockerFieldType::Protocal((i,p)) => format!("{}/{}", i.to_string(), p),
            DockerFieldType::TupleList(l) => l.into_iter()
                                            .map(|(v1,v2)| format!("{}=\"{}\"", v1,v2))
                                            .collect::<Vec<String>>()
                                            .join(" "),
            DockerFieldType::Array(arr) =>  format!("[{}]", 
                                            arr.into_iter()
                                            .map(|i| format!("\"{}\"", i))
                                            .collect::<Vec<String>>()
                                            .join(", ")),
            DockerFieldType::MultiValue(arr) =>  format!("[{}]", 
                                            arr.into_iter()
                                            .map(|i| format!("\"{}\"", i))
                                            .collect::<Vec<String>>()
                                            .join(" ")), 
        }
    }
}

pub struct DockerField<'a>{
    name: &'a str,
    value: DockerFieldType<'a>
}

impl<'a> DockerField<'a> {
    fn new(name: &'a str, value: DockerFieldType<'a>) -> Self{
        DockerField{
            name: name,
            value: value,
        }
    }
}

pub struct DockerFile<'a>{
    fields: Vec<DockerField<'a>>,
}

impl<'a> Default for DockerFile<'a>{
    fn default() -> Self {
        DockerFile{
            fields: Vec::new(),
        }
    }
}

impl<'a> DockerFile<'a>{
    pub fn new() -> Self{
        Default::default()
    }

    pub fn from_image(from: &'a str) -> Self{
        DockerFile::new().from(from)
    }

    pub fn add(mut self,  name: &'a str, val: DockerFieldType<'a>) -> Self {
        self.fields.push(DockerField::new(name, val));
        self
    }

    pub fn comment(self, cmt: &'a str) -> Self{
        self.add("#", cmt.into())
    }

    pub fn from(self, from: &'a str) -> Self {
        self.add("FROM", from.into())
    }

    pub fn from_alias(self, alias: &'a str, from: &'a str) -> Self{
        self.add("FROM", format!("{} as {}", from, alias).into())
    }

    pub fn run(self, from: &'a str) -> Self {
        self.add("RUN", from.into())
    }

    pub fn cmd(self, args: &'a[&'a str]) -> Self{
        self.add("CMD", DockerFieldType::MultiValue(args))
    }

    pub fn label(self, labels: &'a[(&'a str, &'a str)])-> Self{
        self.add("LABEL", labels.into())
    }

    pub fn expose(self, port: i32) -> Self{
        self.add("EXPOSE", port.into())
    }

    pub fn expose_protocal(self, port:i32, prot: &'a str) -> Self{
        self.add("EXPOSE", (port, prot).into())
    }

    pub fn env(self, vals: &'a[(&'a str, &'a str)]) -> Self{
        self.add("ENV", vals.into())
    }

    pub fn workdir(self, dir: &'a str) -> Self{
        self.add("WORKDIR", dir.into())
    }

    pub fn newline(self) -> Self{
        self.add("", "".into())
    }

    pub fn dockerfile(self, f: DockerFile<'a>) -> Self{
        self.add("", f.into())
    }

    pub fn newlines(self, ammount: i32) -> Self{
        let mut s = self;
        for _ in 0..ammount{
            s = s.add("", "".into());
        }
        s
    }


    pub fn to_string(&self) -> String {
        let mut out = String::new();

        for f in self.fields.iter(){
            out.push_str(f.name);
            out.push(' ');
            out.push_str(&f.value.to_string());
            out.push_str("\n\r");
        }
        out
    }
}