use crate::domain::dtos::base::{Page, Pagination};
use crate::domain::enums::Direction;

/// Bazadan qaytgan qatorlarni Page<T> ga aylantiradi.
/// Qarzer: data/repository/helper/PaginationUtils.page()
///
/// Uch qadam: ortiqcha qatorni kesish, kerak bo'lsa ag'darish, kursorlarni o'qish.
///
/// * `rows`       — bazadan kelgan qatorlar (limit + 1 tagacha)
/// * `pagination` — so'rov parametrlari
/// * `cursor_key` — qatordan keyset ustuni qiymatini oladigan funksiya
pub fn to_page<T, F>(mut rows: Vec<T>, pagination: &Pagination, cursor_key: F) -> Page<T>
where
    F: Fn(&T) -> i32,
{
    let limit = pagination.clamped_limit();
    let is_backward = matches!(pagination.direction, Direction::Prev | Direction::Last);
    let uses_cursor = matches!(pagination.direction, Direction::Next | Direction::Prev);

    let has_more = rows.len() > limit as usize;
    if has_more { rows.pop(); }
    if is_backward { rows.reverse(); }

    let came_from_cursor = uses_cursor && pagination.cursor.is_some();
    let has_next = if is_backward { came_from_cursor } else { has_more };
    let has_prev = if is_backward { has_more } else { came_from_cursor };

    let next_cursor = if has_next { rows.last().map(&cursor_key) } else { None };
    let prev_cursor = if has_prev { rows.first().map(&cursor_key) } else { None };

    Page::of(rows, limit, has_next, has_prev, next_cursor, prev_cursor)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::enums::SortDirection;

    // Yordamchi: id 1..=n
    fn asc(from: i32, count: usize) -> Vec<i32> {
        (from..from + count as i32).collect()
    }
    // Yordamchi: id from, from-1, ... (teskari)
    fn desc(from: i32, count: usize) -> Vec<i32> {
        (0..count as i32).map(|i| from - i).collect()
    }
    fn pag(size: u32, cursor: Option<i32>, dir: Direction) -> Pagination {
        Pagination::new(cursor, Some(size), dir)
    }
    fn ids(page: &Page<i32>) -> Vec<i32> {
        page.items().to_vec()
    }

    // ── Oldinga varaqlash (ASC) ───────────────────────────────────────────────

    #[test]
    fn first_page_no_prev_has_next() {
        let page = to_page(asc(1, 4), &pag(3, None, Direction::First), |r| *r);

        assert_eq!(ids(&page), [1, 2, 3]);
        assert!(!page.has_prev());
        assert!(page.has_next());
        assert_eq!(page.prev_cursor(), None);
        assert_eq!(page.next_cursor(), Some(3));
        assert_eq!(page.count(), 3);
        assert_eq!(page.limit(), 3);
    }

    #[test]
    fn middle_page_has_both_cursors() {
        let page = to_page(asc(4, 4), &pag(3, Some(3), Direction::Next), |r| *r);

        assert_eq!(ids(&page), [4, 5, 6]);
        assert!(page.has_prev());
        assert!(page.has_next());
        assert_eq!(page.prev_cursor(), Some(4));
        assert_eq!(page.next_cursor(), Some(6));
    }

    #[test]
    fn last_page_no_next() {
        let page = to_page(asc(10, 1), &pag(3, Some(9), Direction::Next), |r| *r);

        assert_eq!(ids(&page), [10]);
        assert!(!page.has_next());
        assert!(page.has_prev());
        assert_eq!(page.next_cursor(), None);
        assert_eq!(page.prev_cursor(), Some(10));
    }

    #[test]
    fn exactly_limit_means_no_next_page() {
        let page = to_page(asc(1, 3), &pag(3, None, Direction::First), |r| *r);

        assert_eq!(ids(&page), [1, 2, 3]);
        assert!(!page.has_next());
        assert_eq!(page.next_cursor(), None);
    }

    // ── Orqaga varaqlash (ASC) ────────────────────────────────────────────────

    #[test]
    fn prev_page_keeps_natural_order() {
        // Baza DESC tartibda qaytardi: [3, 2, 1, _extra_]
        let page = to_page(desc(3, 4), &pag(3, Some(4), Direction::Prev), |r| *r);

        assert_eq!(ids(&page), [1, 2, 3]); // ag'darilgandan so'ng ASC tartib
        assert!(page.has_next());
        assert!(page.has_prev());
        assert_eq!(page.next_cursor(), Some(3));
        assert_eq!(page.prev_cursor(), Some(1));
    }

    #[test]
    fn reaching_the_beginning_has_prev_false() {
        let page = to_page(desc(3, 3), &pag(3, Some(4), Direction::Prev), |r| *r);

        assert_eq!(ids(&page), [1, 2, 3]);
        assert!(!page.has_prev());
        assert!(page.has_next());
        assert_eq!(page.prev_cursor(), None);
    }

    // ── Chetlarga sakrash (LAST) ──────────────────────────────────────────────

    #[test]
    fn last_page_has_no_next() {
        let page = to_page(desc(10, 4), &pag(3, None, Direction::Last), |r| *r);

        assert_eq!(ids(&page), [8, 9, 10]);
        assert!(!page.has_next());
        assert!(page.has_prev());
        assert_eq!(page.next_cursor(), None);
        assert_eq!(page.prev_cursor(), Some(8));
    }

    // ── Chegara holatlari ─────────────────────────────────────────────────────

    #[test]
    fn empty_result() {
        let page = to_page(vec![], &pag(3, None, Direction::First), |r: &i32| *r);

        assert!(page.items().is_empty());
        assert_eq!(page.count(), 0);
        assert!(!page.has_next());
        assert!(!page.has_prev());
        assert_eq!(page.next_cursor(), None);
        assert_eq!(page.prev_cursor(), None);
    }

    #[test]
    fn next_without_cursor_behaves_as_first() {
        let page = to_page(asc(1, 4), &pag(3, None, Direction::Next), |r| *r);

        assert!(!page.has_prev());
        assert_eq!(ids(&page), [1, 2, 3]);
    }

    #[test]
    fn single_page_dataset_no_cursors() {
        let page = to_page(asc(1, 4), &pag(10, None, Direction::First), |r| *r);

        assert_eq!(ids(&page), [1, 2, 3, 4]);
        assert!(!page.has_next());
        assert!(!page.has_prev());
        assert_eq!(page.next_cursor(), None);
        assert_eq!(page.prev_cursor(), None);
    }

    // ── Kamayish tartibi (DESC lenta) ─────────────────────────────────────────

    #[test]
    fn next_on_desc_feed() {
        // NEXT + DESC: cursor=8, baza id < 8 DESC: [7, 6, 5, 4]
        let p = pag(3, Some(8), Direction::Next).on("id", SortDirection::Desc);
        let page = to_page(desc(7, 4), &p, |r| *r);

        assert_eq!(ids(&page), [7, 6, 5]);
        assert!(page.has_next());
        assert!(page.has_prev());
        assert_eq!(page.next_cursor(), Some(5));
        assert_eq!(page.prev_cursor(), Some(7));
    }

    #[test]
    fn prev_on_desc_feed() {
        // PREV + DESC: cursor=5, baza id > 5 ASC: [6, 7, 8, 9]
        let p = pag(3, Some(5), Direction::Prev).on("id", SortDirection::Desc);
        let page = to_page(asc(6, 4), &p, |r| *r);

        assert_eq!(ids(&page), [8, 7, 6]);
        assert!(page.has_next());
        assert!(page.has_prev());
        assert_eq!(page.next_cursor(), Some(6));
        assert_eq!(page.prev_cursor(), Some(8));
    }

    // ── map() sahifalash ma'lumotini saqlaydi ────────────────────────────────

    #[test]
    fn map_keeps_page_metadata() {
        let page = to_page(asc(4, 4), &pag(3, Some(3), Direction::Next), |r| *r);
        let mapped = page.map(|id| format!("#{id}"));

        assert_eq!(mapped.items(), &["#4", "#5", "#6"]);
        assert!(mapped.has_next());
        assert!(mapped.has_prev());
        assert_eq!(mapped.next_cursor(), Some(6));
        assert_eq!(mapped.prev_cursor(), Some(4));
        assert_eq!(mapped.limit(), 3);
    }

    // ── To'liq varaqlash: 10 ta yozuv ────────────────────────────────────────

    fn simulate_query(all: &[i32], pagination: &Pagination) -> Vec<i32> {
        use crate::domain::enums::SortDirection;
        let limit = pagination.clamped_limit();
        let is_backward = matches!(pagination.direction, Direction::Prev | Direction::Last);
        let ignore_cursor = matches!(pagination.direction, Direction::First | Direction::Last);
        let natural_asc = matches!(pagination.sort(), SortDirection::Asc);
        let sql_asc = is_backward != natural_asc;
        let cursor_gt = natural_asc != is_backward;

        let mut filtered: Vec<i32> = all.iter().copied().filter(|&id| {
            if ignore_cursor { return true; }
            match pagination.cursor {
                None => true,
                Some(c) => if cursor_gt { id > c } else { id < c },
            }
        }).collect();
        if sql_asc { filtered.sort(); } else { filtered.sort_by(|a, b| b.cmp(a)); }
        filtered.truncate(limit as usize + 1);
        filtered
    }

    fn walk(all: &[i32], size: u32, cursor: Option<i32>, dir: Direction) -> Page<i32> {
        let p = pag(size, cursor, dir);
        let rows = simulate_query(all, &p);
        to_page(rows, &p, |r| *r)
    }

    #[test]
    fn walk_forward_four_pages() {
        let all: Vec<i32> = (1..=10).collect();
        let p1 = walk(&all, 3, None, Direction::First);
        assert_eq!(ids(&p1), [1, 2, 3]);
        assert!(!p1.has_prev());

        let p2 = walk(&all, 3, p1.next_cursor(), Direction::Next);
        assert_eq!(ids(&p2), [4, 5, 6]);

        let p3 = walk(&all, 3, p2.next_cursor(), Direction::Next);
        assert_eq!(ids(&p3), [7, 8, 9]);
        assert!(p3.has_next());

        let p4 = walk(&all, 3, p3.next_cursor(), Direction::Next);
        assert_eq!(ids(&p4), [10]);
        assert!(!p4.has_next());
        assert_eq!(p4.next_cursor(), None);
    }

    #[test]
    fn walk_backward_from_last() {
        let all: Vec<i32> = (1..=10).collect();
        let last = walk(&all, 3, None, Direction::Last);
        assert_eq!(ids(&last), [8, 9, 10]);
        assert!(!last.has_next());

        let b1 = walk(&all, 3, last.prev_cursor(), Direction::Prev);
        assert_eq!(ids(&b1), [5, 6, 7]);

        let b2 = walk(&all, 3, b1.prev_cursor(), Direction::Prev);
        assert_eq!(ids(&b2), [2, 3, 4]);
        assert!(b2.has_prev());

        let b3 = walk(&all, 3, b2.prev_cursor(), Direction::Prev);
        assert_eq!(ids(&b3), [1]);
        assert!(!b3.has_prev());
        assert_eq!(b3.prev_cursor(), None);
    }

    #[test]
    fn round_trip_is_stable() {
        let all: Vec<i32> = (1..=10).collect();
        let p1 = walk(&all, 4, None, Direction::First);
        let p2 = walk(&all, 4, p1.next_cursor(), Direction::Next);
        let back = walk(&all, 4, p2.prev_cursor(), Direction::Prev);
        let fwd  = walk(&all, 4, back.next_cursor(), Direction::Next);

        assert_eq!(ids(&p1), [1, 2, 3, 4]);
        assert_eq!(ids(&p2), [5, 6, 7, 8]);
        assert_eq!(ids(&back), ids(&p1));
        assert_eq!(ids(&fwd), ids(&p2));
    }
}
