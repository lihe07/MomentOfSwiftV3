use std::collections::HashMap;
use std::fmt::format;
use std::string::ToString;
use mongodm::prelude::*;
use serde::{Deserialize, Serialize};
use super::ById;
use hmac::Mac;


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
    /// 获取该文件内联模式(`Content Disposition: inline`)传输的预签名链接
    fn generate_signed_inline_url(&self, expires: u32) -> String {
        let mut params = HashMap::new();
        let disposition = format!("inline;filename={};", url_escape::encode_component(&self.name));
        params.insert("response-content-disposition", &*disposition);
        params.insert("response-content-type", self.mime_type.as_str());
        sign(
            "GET",
            &self.path,
            expires,
            params,
            &OSSConfig::default(),
        )
    }
    /// 获取该文件附件模式(`Content Disposition: attachment`)传输的预签名链接
    fn generate_signed_attachment_url(&self, expires: u32) -> String {
        let mut params = HashMap::new();
        let disposition = format!("attachment;filename={};", url_escape::encode_component(&self.name));
        params.insert("response-content-disposition", &*disposition);
        params.insert("response-content-type", &self.mime_type);
        sign(
            "GET",
            &self.path,
            expires,
            params,
            &OSSConfig::default(),
        )
    }
}

struct OSSConfig {
    access_key_id: String,
    access_key_secret: String,
    endpoint: String,
    // oss-cn-hangzhou.aliyuncs.com
    bucket_name: String,
    custom_domain: Option<String>,
}

impl Default for OSSConfig {
    fn default() -> Self {
        OSSConfig {
            access_key_id: crate::config::ACCESS_KEY_ID.to_owned(),
            access_key_secret: crate::config::ACCESS_KEY_SECRET.to_owned(),
            bucket_name: crate::config::BUCKET_NAME.to_owned(),
            endpoint: crate::config::ENDPOINT.to_owned(),
            custom_domain: Some(crate::config::CUSTOM_DOMAIN.to_owned()),
        }
    }
}


fn sign(method: &str, key: &str, expires: u32, params: HashMap<&str, &str>, oss_config: &OSSConfig) -> String {
    let expires_at = chrono::Utc::now() + chrono::Duration::seconds(expires as i64);
    let expires_at = expires_at.timestamp();
    // 生成签名

    let headers_string = "";
    let sub_resource_string = {
        if params.is_empty() {
            "".to_string()
        } else {
            let queries = params.iter().map(|(k, v)| format!("{}={}", k, url_escape::encode_component(v))).collect::<Vec<_>>();
            "?".to_string() + &*queries.join("&")
        }
    };
    let resource_string = format!("/{}/{}{}", oss_config.bucket_name, key, sub_resource_string);

    let date = format!("{}", expires_at);
    let string_to_sign = format!("{}\n{}\n{}\n{}\n{}{}",
                                 method,
                                 "",
                                 "",
                                 date,
                                 headers_string,
                                 resource_string);
    let mut hmac = hmac::Hmac::<sha1::Sha1>::new_from_slice(oss_config.access_key_secret.as_bytes()).unwrap();
    hmac.update(string_to_sign.as_bytes());
    let signature = base64::encode(&hmac.finalize().into_bytes());
    let signature = url_escape::encode_component(&signature);
    let mut params = format!("?OSSAccessKeyId={}&Expires={}&Signature={}{}", oss_config.access_key_id, expires_at, signature, sub_resource_string.replace("?", "&"));
    if let Some(custom_domain) = &oss_config.custom_domain {
        format!("{}/{}{}", custom_domain, key, params)
    } else {
        format!("https://{}.{}/{}{}", oss_config.bucket_name, oss_config.endpoint, key, params)
    }
}


mod tests {
    use std::string::ToString;
    use super::*;

    #[test]
    fn pure_sign() {
        let key = "67e55044-10b1-426f-9247-bb680e5fe0c8";
        let mut params = HashMap::new();
        params.insert("response-content-type", "text/plain");
        params.insert("response-content-disposition", "inline;filename=test.svg");

        dbg!(sign("GET", key, 60, params.to_owned(), &OSSConfig::default()));
        params.insert("response-content-disposition", "attachment;filename=test.svg");
        dbg!(sign("GET", key, 60, params, &OSSConfig::default()));
    }

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

// GET\n\n\n1652098944\n/momentofswift/67e55044-10b1-426f-9247-bb680e5fe0c8
// GET\n\n\n1652099129\n/momentofswift/67e55044-10b1-426f-9247-bb680e5fe0c8
//
// GET\n\n\n1652098889\n\n/momentofswift/67e55044-10b1-426f-9247-bb680e5fe0c8