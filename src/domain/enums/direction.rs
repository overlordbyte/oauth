/// Sahifalash yo'nalishi — qarzer: domain/enums/PaginationDirection
pub enum Direction {
    /// Birinchi sahifa — kursor bo'lsa ham e'tiborga olinmaydi
    First,
    /// Kursordan oldingi sahifa — kursor bo'lmasa Last kabi ishlaydi
    Prev,
    /// Kursordan keyingi sahifa — standart (kursor bo'lmasa First kabi)
    Next,
    /// Oxirgi sahifa — kursor e'tiborga olinmaydi
    Last,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Next
    }
}
