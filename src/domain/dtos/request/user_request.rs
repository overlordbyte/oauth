/// Foydalanuvchilar ro'yxati uchun filter parametrlari.
/// Qarzer: domain/dtos/request/UsersRequestModel
pub struct UserRequest {
    pub name_contains: Option<String>,
    pub email_contains: Option<String>,
}

impl Default for UserRequest {
    fn default() -> Self {
        Self { name_contains: None, email_contains: None }
    }
}
