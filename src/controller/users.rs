//! Foydalanuvchilar uchun REST controller.

use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde::{Deserialize, Serialize};

use crate::domain::{AppError, Direction, Page, Pagination, User, UserRequest};
use crate::state::AppState;

/// Mijozga qaytariladigan foydalanuvchi — `password_hash` ataylab yo'q.
#[derive(Serialize)]
pub struct UserBody {
    id: i32,
    name: String,
    email: String,
}

impl From<User> for UserBody {
    fn from(u: User) -> Self {
        Self { id: u.id, name: u.name, email: u.email }
    }
}

/// Keyset sahifalash meta-ma'lumoti bilan ro'yxat javobi.
#[derive(Serialize)]
pub struct PageBody<T> {
    items: Vec<T>,
    count: usize,
    limit: i64,
    has_next: bool,
    has_prev: bool,
    next_cursor: Option<i32>,
    prev_cursor: Option<i32>,
}

impl<T> From<Page<T>> for PageBody<T> {
    fn from(page: Page<T>) -> Self {
        let (count, limit) = (page.count(), page.limit());
        let (has_next, has_prev) = (page.has_next(), page.has_prev());
        let (next_cursor, prev_cursor) = (page.next_cursor(), page.prev_cursor());
        Self {
            items: page.into_items(),
            count,
            limit,
            has_next,
            has_prev,
            next_cursor,
            prev_cursor,
        }
    }
}

/// `GET /api/v1/users` query parametrlari.
#[derive(Deserialize, Default)]
pub struct ListQuery {
    /// Shu id dan keyingi sahifa
    after_id: Option<i32>,
    /// Shu id dan oldingi sahifa (berilsa `after_id` e'tiborga olinmaydi)
    before_id: Option<i32>,
    limit: Option<u32>,
    name_contains: Option<String>,
    email_contains: Option<String>,
}

pub async fn list(
    State(state): State<AppState>,
    Query(q): Query<ListQuery>,
) -> Result<Json<PageBody<UserBody>>, AppError> {
    // before_id berilgan bo'lsa — orqaga yo'nalish
    let (cursor, direction) = match q.before_id {
        Some(before) => (Some(before), Direction::Prev),
        None => (q.after_id, Direction::Next),
    };

    let pagination = Pagination::new(cursor, q.limit, direction);
    let params = UserRequest {
        name_contains: q.name_contains,
        email_contains: q.email_contains,
    };

    let page = state.users.list(&pagination, &params).await?;
    Ok(Json(page.map(UserBody::from).into()))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<UserBody>, AppError> {
    let user = state.users.get(id).await?;
    Ok(Json(user.into()))
}
