/// Tartiblash yo'nalishi — qarzer: domain/enums/SortDirection
pub enum SortDirection {
    Asc,
    Desc,
}

impl Default for SortDirection {
    fn default() -> Self {
        Self::Asc
    }
}
