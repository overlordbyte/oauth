pub mod proto {
    tonic::include_proto!("users");
}

use tonic::{Request, Response, Status};

use proto::users_server::UsersServer;
use proto::{GetUserRequest, ListUsersRequest, ListUsersResponse, User};

use crate::domain::{Direction, Pagination, UserRequest};
use crate::state::AppState;

pub struct UsersService {
    state: AppState,
}

impl UsersService {
    fn new(state: AppState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl proto::users_server::Users for UsersService {
    async fn get_user(&self, req: Request<GetUserRequest>) -> Result<Response<User>, Status> {
        let user = self.state.users
            .get(req.into_inner().id as i32)
            .await
            .map_err(super::to_status)?;
        Ok(Response::new(User { id: user.id as u32, name: user.name, email: user.email }))
    }

    async fn list_users(&self, req: Request<ListUsersRequest>) -> Result<Response<ListUsersResponse>, Status> {
        let r = req.into_inner();
        // before_id mavjud → Prev yo'nalish; before_id=0 va after_id yo'q → First
        let (cursor, direction) = if let Some(before) = r.before_id {
            (Some(before), Direction::Prev)
        } else {
            (r.after_id, Direction::Next)
        };
        let limit = if r.limit == 0 { None } else { Some(r.limit) };
        let pagination = Pagination::new(cursor, limit, direction);
        let params = UserRequest { name_contains: r.name_contains, email_contains: r.email_contains };
        let page = self.state.users.list(&pagination, &params).await.map_err(super::to_status)?;
        let (next_cursor, has_next, has_prev, prev_cursor, count, limit) = (
            page.next_cursor(), page.has_next(), page.has_prev(),
            page.prev_cursor(), page.count(), page.limit(),
        );
        let users = page.into_items().into_iter()
            .map(|u| User { id: u.id as u32, name: u.name, email: u.email })
            .collect();
        Ok(Response::new(ListUsersResponse {
            users,
            next_cursor,
            has_next,
            has_prev,
            prev_cursor,
            count: count as u32,
            limit: limit as u32,
        }))
    }
}

pub fn service(state: AppState) -> UsersServer<UsersService> {
    UsersServer::new(UsersService::new(state))
}
