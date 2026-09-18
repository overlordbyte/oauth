/// Cross-cutting concerns — transport qatlamiga tegishli middleware lar.
///
/// Domain va usecase qatlamlari bu moduldan MUTLAQO BEXABAR —
/// bu qarama-qarshi yo'nalishli bog'liqlikni oldini oladi (clean arch).
pub mod cors;
pub mod rate_limit;
