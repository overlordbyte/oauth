/// Foydalanuvchilar ro'yxati uchun filter parametrlari.
/// Qarzer: domain/dtos/request/UsersRequestModel
#[derive(Default)]
pub struct UserRequest {
    pub name_contains: Option<String>,
    pub email_contains: Option<String>,
}

