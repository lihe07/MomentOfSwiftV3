use mongodm::prelude::*;
use serde::{Deserialize, Serialize};
use super::ById;

pub struct CollConf;

impl CollectionConfig for CollConf {
    fn collection_name() -> &'static str {
        "files" // collection name
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
pub struct File {
    pub id: uuid::Uuid,
    // ...
    pub name: String,
    // 原始文件名
    pub path: String,
    // OSS存储路径
    pub last_modified_time: bson::DateTime,
    // 上次修改时间
    pub created_time: bson::DateTime,
    // 创建时间
    pub mime_type: String,
    // MIME类型
    pub owner: uuid::Uuid, // 文件所有者
}

impl Model for File {
    type CollConf = CollConf;
}

impl ById for File {}

impl File {
    /// 获取该文件内联模式(`Content Deposition: inline`)传输的预签名链接
    fn generate_signed_inline_url(&self, expires: u64) -> String {
        todo!()
    }
    /// 获取该文件附件模式(`Content Deposition: attachment`)传输的预签名链接
    fn generate_signed_attachment_url(&self, expires: u64) -> String {
        todo!()
    }
}

mod tests {
    use super::*;

    #[async_std::test]
    async fn generate_signature() -> anyhow::Result<()> {
        let client = mongodm::mongo::Client::with_options(
            mongodm::mongo::options::ClientOptions::parse(crate::config::CONNECTION_STRING).await?,
        )?;
        let db = client.database(crate::config::DATABASE_NAME);
        let repo = db.repository::<File>();
        let file = File::by_id(&repo, "SOME_ID").await;
        if let Some(file) = file {
            let signed_inline_url = file.generate_signed_inline_url(3600);
            let signed_attachment_url = file.generate_signed_attachment_url(3600);
            println!("signed_inline_url: {}", signed_inline_url);
            println!("signed_attachment_url: {}", signed_attachment_url);
        } else {
            println!("file not found");
        }
        Ok(())
    }
}