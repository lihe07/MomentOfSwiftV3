use std::string::ToString;
use mongodm::prelude::*;
use serde::{Deserialize, Serialize};
use crate::config::DB;

pub struct CollConf;

impl CollectionConfig for CollConf {
    fn collection_name() -> &'static str {
        "groups" // collection name
    }

    // fn collection_options() -> Option<MongoCollectionOptions> {
    //
    // }

    fn indexes() -> Indexes {
        Indexes::new()
            .with(Index::new("id").with_option(IndexOption::Unique)) // id字段不允许重复
            .with(Index::new("name").with_option(IndexOption::Unique)) // 组名不允许重复
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, data_model_macro::ToJson, data_model_macro::ById)]
pub struct Group {
    pub id: uuid::Uuid,
    pub name: String,
    pub category: i64,
    // 可选字段
    pub cover: Option<String>,
    pub profile: Option<String>,
}

impl Model for Group {
    type CollConf = CollConf;
}


/// 公共方法实现
impl Group {
    /// 某个name是否存在
    pub async fn name_exists<T: Into<String>>(name: T) -> anyhow::Result<bool> {
        let repo = DB.repository::<Group>();
        Ok(repo.count_documents(doc! {"name": name.into()}, None).await? > 0)
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
