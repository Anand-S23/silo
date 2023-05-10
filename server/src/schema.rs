use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Default)]
pub struct CreateTestSchema {
    pub fname: String,
    pub lname: String
}
