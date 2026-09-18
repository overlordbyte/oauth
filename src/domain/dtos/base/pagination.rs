use crate::domain::enums::{Direction, SortDirection};

/// Keyset pagination so'rovi — qarzer: domain/dtos/base/KeysetPagination
///
/// Faqat NIYATNI tasvirlaydi; uni SQL'ga o'girish store qatlamining ishi.
pub struct Pagination {
    pub cursor: Option<i32>,
    pub size: Option<u32>,
    pub direction: Direction,
    /// Keyset ustuni (server tomonidan on() bilan o'rnatiladi)
    column: String,
    /// Tabiiy tartiblash yo'nalishi
    sort: SortDirection,
}

impl Pagination {
    pub const MIN_LIMIT: u32 = 1;
    pub const DEFAULT_LIMIT: u32 = 20;
    pub const MAX_LIMIT: u32 = 50;

    /// Limitni ruxsat etilgan oraliqqa keltiradi.
    pub fn clamped_limit(&self) -> i64 {
        self.size
            .unwrap_or(Self::DEFAULT_LIMIT)
            .clamp(Self::MIN_LIMIT, Self::MAX_LIMIT) as i64
    }

    /// Keyset ustuni va tartiblash yo'nalishini server tomonida o'rnatadi.
    /// Mijoz bu qiymatlarni o'zgartira olmaydi.
    pub fn on(mut self, column: impl Into<String>, sort: SortDirection) -> Self {
        self.column = column.into();
        self.sort = sort;
        self
    }

    pub fn column(&self) -> &str { &self.column }
    pub fn sort(&self) -> &SortDirection { &self.sort }

    pub fn new(cursor: Option<i32>, size: Option<u32>, direction: Direction) -> Self {
        Self { cursor, size, direction, column: "id".into(), sort: SortDirection::default() }
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self::new(None, None, Direction::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── clamped_limit chegaralari ─────────────────────────────────────────────

    #[test]
    fn null_size_falls_back_to_default() {
        let p = Pagination::new(None, None, Direction::Next);
        assert_eq!(p.clamped_limit(), Pagination::DEFAULT_LIMIT as i64);
    }

    #[test]
    fn zero_size_is_raised_to_min_limit() {
        let p = Pagination::new(None, Some(0), Direction::Next);
        assert_eq!(p.clamped_limit(), Pagination::MIN_LIMIT as i64);
    }

    #[test]
    fn oversized_is_capped_at_max_limit() {
        for &size in &[51u32, 100, u32::MAX] {
            let p = Pagination::new(None, Some(size), Direction::Next);
            assert_eq!(p.clamped_limit(), Pagination::MAX_LIMIT as i64,
                "size={size} should be capped");
        }
    }

    #[test]
    fn valid_size_is_kept() {
        let p = Pagination::new(None, Some(7), Direction::Next);
        assert_eq!(p.clamped_limit(), 7);
    }

    #[test]
    fn boundaries_are_inclusive() {
        let min_p = Pagination::new(None, Some(Pagination::MIN_LIMIT), Direction::Next);
        let max_p = Pagination::new(None, Some(Pagination::MAX_LIMIT), Direction::Next);
        assert_eq!(min_p.clamped_limit(), Pagination::MIN_LIMIT as i64);
        assert_eq!(max_p.clamped_limit(), Pagination::MAX_LIMIT as i64);
    }

    // ── standart qiymatlar ────────────────────────────────────────────────────

    #[test]
    fn defaults_are_applied() {
        let p = Pagination::default();
        assert!(matches!(p.direction, Direction::Next));
        assert!(matches!(p.sort(), SortDirection::Asc));
        assert_eq!(p.column(), "id");
        assert_eq!(p.cursor, None);
    }

    // ── on() ─────────────────────────────────────────────────────────────────

    #[test]
    fn on_keeps_the_rest_of_the_request() {
        let original = Pagination::new(Some(42), Some(15), Direction::Prev);
        let moved = original.on("message_id", SortDirection::Desc);

        assert_eq!(moved.clamped_limit(), 15);
        assert_eq!(moved.cursor, Some(42));
        assert!(matches!(moved.direction, Direction::Prev));
        assert_eq!(moved.column(), "message_id");
        assert!(matches!(moved.sort(), SortDirection::Desc));
    }
}
