#[derive(Debug)]
pub struct JavaDomain {
    pub name: String,
    pub objects: Vec<JavaObject>,
}

#[derive(Debug)]
pub struct JavaObject {
    pub class: String,
    pub variable_name: String,
}