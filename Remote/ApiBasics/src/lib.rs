use std::error::Error;
use data_model::ToJson;
use serde::Serialize;
use serde_json::{json, Value};

/// 将一个实现了ToJson trait的结构体转换为响应，其中响应状态为200
pub async fn make_response<T: ToJson>(data: T) -> tide::Result {
    Ok(tide::Response::from(data.to_json().await))
}

/// 将任意实现了Serialize的结构体转换为响应，需自行决定状态
pub fn json_response<T: Serialize, S: TryInto<tide::StatusCode>>(data: T, code: S) -> tide::Result {
    match serde_json::to_value(data) {
        Ok(value) => {
            let mut resp = tide::Response::new(code);
            resp.set_body(value);
            Ok(resp)
        }
        Err(err) => make_error("serialization", None, Some(err.into()))
    }
}

/// 制作一个标准错误响应
pub fn make_error<STR: AsRef<&str>, T: Serialize>(name: STR, details: Option<T>, error: Option<anyhow::Error>) -> tide::Result {
    if let Some(err) = error {
        let value = if let Some(details) = details {
            json!({
            "error": name,
            "backtrace": err.backtrace(),
                "details": details
            })
        } else {
            json!({
                "error": name,
                "backtrace": err.backtrace(),
            })
        };
        let mut resp = tide::Response::new(500);
        resp.set_body(value);
        Ok(resp)
    } else {
        let value = if let Some(details) = details {
            json!({
                "error": name,
                "details": details
            })
        } else {
            json!({
                "error": name
            })
        };
        let mut resp = tide::Response::new(400);
        resp.set_body(value);
        Ok(resp)
    }
}

pub async fn paginate<T: ToJson>(cursor: mongodb::Cursor<T>, limit: usize, page: usize) -> tide::Result {
    let mut elements = Vec::new();
    let mut i = 0;
    let total = cursor.count();
    let from = limit * page;
    let to = from + limit;
    while let Some(model) = cursor.next() {
        if i >= from && i <= to {
            elements.push(some)
        }
        i += 1;
    }
    json_response(json!({
        "elements": elements,
        "total": total
    }), 200)
}

