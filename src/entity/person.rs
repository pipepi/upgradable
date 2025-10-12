use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub email: String,
}
