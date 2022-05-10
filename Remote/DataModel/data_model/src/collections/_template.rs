use bson::oid::ObjectId;
use mongodm::prelude::*;
use serde::{Deserialize, Serialize};

pub struct CollConf;

impl CollectionConfig for CollConf {
    fn collection_name() -> &'static str {
        "my_collection" // collection name
    }

    // fn collection_options() -> Option<MongoCollectionOptions> {
    //
    // }

    fn indexes() -> Indexes {
        // 创建索引, id字段不允许重复
        Indexes::new()
            .with(Index::new("id").with_option(IndexOption::Unique))
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TemplateModel {
    pub id: uuid::Uuid,
    // ...
}

impl Model for TemplateModel {
    type CollConf = CollConf;
}

impl TemplateModel {
    // ...
}

mod tests {
    use super::*;

    #[async_std::test]
    async fn insert_model() -> anyhow::Result<()> {
        let client = mongodm::mongo::Client::with_options(
            mongodm::mongo::options::ClientOptions::parse(crate::config::CONNECTION_STRING).await?,
        )?;
        let db = client.database("db_name");
        let repo = db.repository::<TemplateModel>();
        repo.insert_one(
            TemplateModel {
                id: uuid::Uuid::new_v4(),
            }
            , None).await?;
        Ok(())
    }
}