/// Barcha output portlar uchun marker trait
///
/// UseCase layeri portni ushbu trait orqali taniydi.
/// Store layeri bu traitni implement qiladi.
pub trait Port: Send + Sync {}
