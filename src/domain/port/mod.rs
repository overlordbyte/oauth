/// Barcha output portlar uchun marker trait — qarzer: Repository (marker interface)
///
/// UseCase layeri portni ushbu trait orqali taniydi.
/// Store layeri bu traitni implement qiladi.
pub trait Port: Send + Sync {}
