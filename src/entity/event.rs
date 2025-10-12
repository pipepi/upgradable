use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub id: u32,
    pub name: String,
    pub location: String,
    pub date: String,
}
