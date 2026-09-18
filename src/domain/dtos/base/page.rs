/// Bitta sahifaning natijasi: qatorlar va sahifalash meta-ma'lumoti.
/// Qarzer: domain/dtos/base/KeysetPage<T>
///
/// Bu HTTP javobi emas — na code, na status bor.
/// Controller uni proto javobiga o'giradi.
pub struct Page<T> {
    items: Vec<T>,
    /// Qaytgan qatorlar soni (items.len())
    count: usize,
    /// Amalda qo'llangan limit
    limit: i64,
    /// Keyingi sahifa bormi
    has_next: bool,
    /// Oldingi sahifa bormi
    has_prev: bool,
    /// Keyingi sahifaning kursori (has_next=false bo'lsa None)
    next_cursor: Option<i32>,
    /// Oldingi sahifaning kursori (has_prev=false bo'lsa None)
    prev_cursor: Option<i32>,
}

impl<T> Page<T> {
    /// Factory — bo'sh sahifada kursorlar majburiy None bo'ladi.
    pub fn of(
        items: Vec<T>,
        limit: i64,
        has_next: bool,
        has_prev: bool,
        next_cursor: Option<i32>,
        prev_cursor: Option<i32>,
    ) -> Self {
        let empty = items.is_empty();
        let count = items.len();
        Self {
            items,
            count,
            limit,
            has_next,
            has_prev,
            // Bo'sh sahifada kursor bo'lishi MUMKIN emas
            next_cursor: if empty { None } else { next_cursor },
            prev_cursor: if empty { None } else { prev_cursor },
        }
    }

    /// Bo'sh sahifa — na oldin, na keyin qator bor.
    pub fn empty(limit: i64) -> Self {
        Self::of(vec![], limit, false, false, None, None)
    }

    /// Elementlarni boshqa turga o'giradi, sahifalash meta-ma'lumotini SAQLAB.
    pub fn map<R, F: Fn(T) -> R>(self, f: F) -> Page<R> {
        Page {
            items: self.items.into_iter().map(f).collect(),
            count: self.count,
            limit: self.limit,
            has_next: self.has_next,
            has_prev: self.has_prev,
            next_cursor: self.next_cursor,
            prev_cursor: self.prev_cursor,
        }
    }

    pub fn items(&self) -> &[T] { &self.items }
    pub fn into_items(self) -> Vec<T> { self.items }
    pub fn count(&self) -> usize { self.count }
    pub fn limit(&self) -> i64 { self.limit }
    pub fn has_next(&self) -> bool { self.has_next }
    pub fn has_prev(&self) -> bool { self.has_prev }
    pub fn next_cursor(&self) -> Option<i32> { self.next_cursor }
    pub fn prev_cursor(&self) -> Option<i32> { self.prev_cursor }
}
