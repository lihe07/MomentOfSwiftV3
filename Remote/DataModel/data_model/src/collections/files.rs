use std::string::ToString;
use mongodm::prelude::*;
use serde::{Deserialize, Serialize};
use hmac::Mac;
use urlqstring::QueryParams;


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

#[derive(Serialize, Deserialize, Debug, PartialEq, data_model_macro::ToJson, data_model_macro::ById)]
pub struct File {
    pub id: uuid::Uuid,
    // ...
    pub name: String,
    // 原始文件名
    #[hidden] // 不在响应中体现
    pub path: String,
    // OSS存储路径
    pub last_modified_time: bson::DateTime,
    // 上次修改时间
    pub created_time: bson::DateTime,
    // 创建时间
    pub mime_type: String,
    // MIME类型
    #[expand(model = File)]
    pub owner: uuid::Uuid, // 文件所有者
}

impl Model for File {
    type CollConf = CollConf;
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::{ById, ToJson};

    // #[test]
    // fn pure_sign() {
    //     let key = "67e55044-10b1-426f-9247-bb680e5fe0c8";
    //     // 这里要注意百分号编码
    //     let params = vec![
    //         ("response-content-disposition", "inline;filename=%E4%B8%AD%E6%96%87%E5%90%8D.svg;"),
    //         ("response-content-type", "text/plain"),
    //     ];
    //
    //     dbg!(sign("GET", key, 600, params.to_owned(), &OSSConfig::default()));
    //     let params = vec![
    //         ("response-content-disposition", "attachment;filename=%E4%B8%AD%E6%96%87%E5%90%8D.svg;"),
    //         ("response-content-type", "text/plain"),
    //     ];
    //     dbg!(sign("GET", key, 600, params, &OSSConfig::default()));
    // }
    //
    // #[async_std::test]
    // async fn generate_signature() -> anyhow::Result<()> {
    //     let file = File::by_id("SOME_ID").await;
    //     if let Some(file) = file {
    //         let signed_inline_url = file.generate_signed_inline_url(3600);
    //         let signed_attachment_url = file.generate_signed_attachment_url(3600);
    //         println!("signed_inline_url: {}", signed_inline_url);
    //         println!("signed_attachment_url: {}", signed_attachment_url);
    //     } else {
    //         println!("file not found");
    //     }
    //     Ok(())
    // }

    #[test]
    fn test_hidden_attribute() {
        let file = File {
            id: uuid::Uuid::new_v4(),

            name: "name".to_string(),
            path: "path".to_string(),
            last_modified_time: chrono::Utc::now().into(),
            created_time: chrono::Utc::now().into(),
            mime_type: "mime_type".to_string(),
            owner: uuid::Uuid::new_v4(),
        };
        // dbg!(serde_json::to_value(file));
        let result: serde_json::Value = file.to_json_without_expand();
        if let Some(_) = result.get("path") {
            panic!("#[hidden] 标签没有生效");
        }
    }
}
