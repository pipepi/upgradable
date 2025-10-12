use std::error::Error;
// mod entity;
// use entity::event::Event;
use upgradable::json_store_to_file;
fn main() -> Result<(), Box<dyn Error>> {
    // let event = Event {
    //     id: 1,
    //     name: "Concert".to_string(),
    //     location: "Stadium".to_string(),
    //     date: "2023-09-15".to_string(),
    // };
    // let event_json = serde_json::to_string_pretty(&event)?;
    // println!("Event JSON:\n{}", event_json);
    let r = json_store_to_file();
    return r;
}
