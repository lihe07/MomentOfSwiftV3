use std::string::ToString;
use mongodm::prelude::*;
use serde::{Deserialize, Serialize};
use crate::config::DB;


pub struct CollConf;

impl CollectionConfig for CollConf {
    fn collection_name() -> &'static str {
        "users" // collection name
    }

    // fn collection_options() -> Option<MongoCollectionOptions> {
    //
    // }

    fn indexes() -> Indexes {
        // 创建索引
        Indexes::new()
            .with(Index::new("id").with_option(IndexOption::Unique)) // 用户id不允许重复
            .with(Index::new("name").with_option(IndexOption::Unique)) // 真实姓名不允许重复
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, data_model_macro::ToJson, data_model_macro::ById)]
pub struct User {
    // 必备字段
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub permission: Vec<String>,
    // 可选字段
    pub avatar: Option<String>,
    pub profile: Option<String>,
}

impl Model for User {
    type CollConf = CollConf;
}

// 公共方法
impl User {
    /// 通过邮箱查找多个用户
    pub async fn by_email<T: Into<String>>(email: T) -> anyhow::Result<Self> {
        let repo = DB.repository();
        let cursor = repo.find(doc! {"email": email.into()}, None).await?;
        cursor.collect().await
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // #[async_std::test]
    // fn it_works() {
    //     // ...
    // }
}
