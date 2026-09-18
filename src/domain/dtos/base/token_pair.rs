/// Access va Refresh token juftligi
#[derive(Debug, Clone)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    /// Access token muddati (soniyalarda)
    pub expires_in: u64,
}
