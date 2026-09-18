/// Sahifalash yo'nalishi/enums/PaginationDirection
#[derive(Default)]
pub enum Direction {
    /// Birinchi sahifa — kursor bo'lsa ham e'tiborga olinmaydi
    First,
    /// Kursordan oldingi sahifa — kursor bo'lmasa Last kabi ishlaydi
    Prev,
    /// Kursordan keyingi sahifa — standart (kursor bo'lmasa First kabi)
    #[default]
    Next,
    /// Oxirgi sahifa — kursor e'tiborga olinmaydi
    Last,
}

