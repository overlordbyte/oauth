use serde::{Deserialize, Serialize};

// Backend hali profil endpointini bermaydi — model shu paytgacha zaxirada turadi.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}
