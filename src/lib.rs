pub mod entity;
use entity::person::Person;
use std::error::Error;
pub fn json_store_to_file() -> Result<(), Box<dyn Error>> {
    let person = Person {
        name: "Alice".to_string(),
        age: 40,
        email: "alice@example.com".to_string(),
    };
    let json_data = serde_json::to_string_pretty(&person)?;
    std::fs::write("person.json", json_data)?;
    println!("Person data written to person.json");

    Ok(())
}
