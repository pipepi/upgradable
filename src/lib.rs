pub mod entity;
use entity::person::Person;
use std::error::Error;
use std::fs;
use std::path::Path;
pub fn json_store_to_file() -> Result<(), Box<dyn Error>> {
    let person = Person {
        name: "Alice".to_string(),
        age: 40,
        email: "alice@example.com".to_string(),
    };
    let file_path = "./doc/temp/run_output/person.json";

    // Create parent directories if they don't exist
    if let Some(parent) = Path::new(file_path).parent() {
        fs::create_dir_all(parent)?;
    }
    let json_data = serde_json::to_string_pretty(&person)?;
    std::fs::write(file_path, json_data)?;
    println!("Person data written to {}", file_path);

    Ok(())
}
