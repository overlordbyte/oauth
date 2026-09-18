use async_trait::async_trait;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl, pooled_connection::bb8::Pool};

use crate::domain::{AppError, Direction, Page, Pagination, Port, SortDirection, User, UserRequest};
use crate::store::helper::pagination::to_page;
use crate::store::schema::users as t;
use crate::usecase::users::UsersPort;

// ── DB models (private — Diesel implementation detail) ────────────────────────

#[derive(Queryable, Selectable)]
#[diesel(table_name = t)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct UserDb {
    id: i32,
    name: String,
    email: String,
    password_hash: String,
}

impl From<UserDb> for User {
    fn from(db: UserDb) -> Self {
        Self { id: db.id, name: db.name, email: db.email, password_hash: db.password_hash }
    }
}

#[derive(Insertable)]
#[diesel(table_name = t)]
struct NewUserDb {
    name: String,
    email: String,
    password_hash: String,
}

// ── Repository implementation ─────────────────────────────────────────────────

pub struct PgUsersStore {
    pool: Pool<AsyncPgConnection>,
}

impl PgUsersStore {
    pub fn new(pool: Pool<AsyncPgConnection>) -> Self {
        Self { pool }
    }
}

impl Port for PgUsersStore {}

#[async_trait]
impl UsersPort for PgUsersStore {
    async fn find_by_id(&self, id: i32) -> Result<Option<User>, AppError> {
        let mut conn = self.pool.get().await.map_err(|e| AppError::Internal(e.to_string()))?;
        t::table
            .find(id)
            .select(UserDb::as_select())
            .first(&mut conn)
            .await
            .optional()
            .map(|opt| opt.map(User::from))
            .map_err(|e| AppError::Internal(e.to_string()))
    }

    async fn find_all(&self, pagination: &Pagination, params: &UserRequest) -> Result<Page<User>, AppError> {
        let mut conn = self.pool.get().await.map_err(|e| AppError::Internal(e.to_string()))?;
        let limit = pagination.clamped_limit();

        // FIRST/LAST → kursor e'tiborga olinmaydi; PREV/LAST → teskari tartib
        let is_backward = matches!(pagination.direction, Direction::Prev | Direction::Last);
        let ignore_cursor = matches!(pagination.direction, Direction::First | Direction::Last);
        // natural_asc: ro'yxatning tabiiy tartibi (SortDirection dan)
        // sql_asc: bazaga yuboriladigan ORDER BY yo'nalishi (backward bo'lsa teskari)
        // Qarzer: PaginationUtils.apply() → keysetAsc = backward != sortAscending
        let natural_asc = matches!(pagination.sort(), SortDirection::Asc);
        let sql_asc = is_backward != natural_asc;
        // Kursor taqqoslash: (asc va forward) YOKI (desc va backward) → gt; aks holda lt
        let cursor_gt = natural_asc != is_backward;

        // Har yo'nalish uchun alohida query (Diesel type inference muammosi)
        let rows: Vec<UserDb> = if sql_asc {
            let mut q = t::table.select(UserDb::as_select()).into_boxed();
            if !ignore_cursor && let Some(c) = pagination.cursor {
                if cursor_gt { q = q.filter(t::id.gt(c)); } else { q = q.filter(t::id.lt(c)); }
            }
            if let Some(n) = &params.name_contains { q = q.filter(t::name.ilike(format!("%{n}%"))); }
            if let Some(e) = &params.email_contains { q = q.filter(t::email.ilike(format!("%{e}%"))); }
            q.order(t::id.asc()).limit(limit + 1).load::<UserDb>(&mut conn).await
                .map_err(|e| AppError::Internal(e.to_string()))?
        } else {
            let mut q = t::table.select(UserDb::as_select()).into_boxed();
            if !ignore_cursor && let Some(c) = pagination.cursor {
                if cursor_gt { q = q.filter(t::id.gt(c)); } else { q = q.filter(t::id.lt(c)); }
            }
            if let Some(n) = &params.name_contains { q = q.filter(t::name.ilike(format!("%{n}%"))); }
            if let Some(e) = &params.email_contains { q = q.filter(t::email.ilike(format!("%{e}%"))); }
            q.order(t::id.desc()).limit(limit + 1).load::<UserDb>(&mut conn).await
                .map_err(|e| AppError::Internal(e.to_string()))?
        };

        Ok(to_page(rows, pagination, |r| r.id).map(User::from))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let mut conn = self.pool.get().await.map_err(|e| AppError::Internal(e.to_string()))?;
        t::table
            .filter(t::email.eq(email))
            .select(UserDb::as_select())
            .first(&mut conn)
            .await
            .optional()
            .map(|opt| opt.map(User::from))
            .map_err(|e| AppError::Internal(e.to_string()))
    }

    async fn create(&self, name: String, email: String, password_hash: String) -> Result<User, AppError> {
        let mut conn = self.pool.get().await.map_err(|e| AppError::Internal(e.to_string()))?;
        diesel::insert_into(t::table)
            .values(NewUserDb { name, email, password_hash })
            .returning(UserDb::as_returning())
            .get_result(&mut conn)
            .await
            .map(User::from)
            .map_err(|e| {
                if let diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation, _
                ) = &e {
                    AppError::AlreadyExists { entity: "user", field: "email" }
                } else {
                    AppError::Internal(e.to_string())
                }
            })
    }
}
