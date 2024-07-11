use std::fs;
use std::fs::File;
use std::io::Write;
use crate::domain::JavaDomain;

pub fn run(domain: JavaDomain){
    let name = String::from(&domain.name);

    create_directory_if_not_exists(&name);
    generate_dto(&domain, &domain.name).expect("Could not write to file");
}

fn create_directory_if_not_exists(path: &str){
    if let Err(e) = fs::create_dir_all(path) {
        println!("Failed to create directory: {}", e);
    } else {
        println!("Directory created successfully");
    }
}

fn generate_dto(java_domain: &JavaDomain, path: &str) -> std::io::Result<()>{
    let path = format!("{}/Rest{}Dto.java", path, java_domain.name);

    let mut file = File::create(path)?;

    let mut data = format!("{}", "@Schema(name = \"StationDto\")");
    data.push_str("\n@Builder");
    data.push_str("\n@Jacksonized");
    data.push_str(&format!("\npublic record Rest{}Dto(", java_domain.name));

    for object in &java_domain.objects {
        data.push_str(&format!("\n\t\t{} {},", object.class, object.variable_name));
    }

    data.push_str("\n) {");
    data.push_str("\n}");

    file.write_all(data.as_bytes())?;

    Ok(())
}