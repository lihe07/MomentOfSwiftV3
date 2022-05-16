use std::fmt::Debug;
use async_mongodb_session::MongodbSessionStore;
use tide::{Body, Middleware, Next, Request, Response, StatusCode};
use tide::sessions::SessionMiddleware;
use tide::utils::After;
use data_model::ById;
use tide::prelude::json;

mod config;

/// 请求附加信息
/// 描述了当前请求的发起者
pub struct RequestExt {
    pub user: Option<data_model::User>, // 如果登录了，则有值
}

struct RequestExtMiddleware;

#[tide::utils::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for RequestExtMiddleware {
    async fn handle(&self, mut request: Request<State>, next: Next<'_, State>) -> tide::Result {
        let session = request.session_mut();
        if let Some(user_id) = session.get::<String>("login") {
            if let Some(user) = data_model::User::by_id(&user_id).await {
                let ext = RequestExt {
                    user: Some(user),
                };
                request.set_ext(ext);
            } else {
                session.remove("login");
                // 这里应该返回一个错误
                return response(401, json!({
                    "code": "session.invalid_user",
                    "details": {
                        "invalid_id": user_id,
                    }
                }));
            }
        } else {
            let ext = RequestExt {
                user: None,
            };
            request.set_ext(ext);
        }
        Ok(next.run(request).await)
    }
}

pub fn response<S>(code: S, data: impl Into<Body>) -> tide::Result
    where
        S: TryInto<StatusCode>, S::Error: Debug
{
    let mut resp = Response::new(code);
    resp.set_body(data);
    Ok(resp)
}

async fn error_handler(resp: Response) -> tide::Result {
    if let Some(err) = resp.downcast_error::<std::io::Error>() {
        return response(500, json!({
            "code": "server.io_error",
            "details": {
                "error": err.to_string(),
            }
        }));
    }

    if resp.status() == StatusCode::NotFound && resp.is_empty().unwrap_or(true) {
        return response(404, json!({
            "code": "server.not_found",
        }));
    }

    Ok(resp)
}


pub async fn init_middlewares<State>(mut app: tide::Server<State>) -> tide::Server<State>
    where State: Clone + Send + Sync + 'static
{
    app.with(SessionMiddleware::new(
        MongodbSessionStore::new(config::CONNECTION_STRING, config::DATABASE_NAME, config::SESSIONS_COLLECTION).await.unwrap(),
        config::SESSION_SECRET.as_bytes(),
    ));

    app.with(RequestExtMiddleware {});
    app.with(After(error_handler));
    app
}

