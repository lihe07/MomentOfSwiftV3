fn generate_signed_inline_url(&self, expires: u32) -> String {
    let disposition = format!("inline;filename={};", url_escape::encode_component(&self.name));
    sign(
        "GET",
        &self.path,
        expires,
        vec![
            ("response-content-disposition", &*disposition),
            ("response-content-type", self.mime_type.as_str()),
        ],
        &OSSConfig::default(),
    )
}
/// 获取该文件附件模式(`Content Disposition: attachment`)传输的预签名链接
fn generate_signed_attachment_url(&self, expires: u32) -> String {
    let disposition = format!("attachment;filename={};", url_escape::encode_component(&self.name));
    sign(
        "GET",
        &self.path,
        expires,
        vec![
            ("response-content-disposition", &*disposition),
            ("response-content-type", self.mime_type.as_str()),
        ],
        &OSSConfig::default(),
    )
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


fn sign(method: &str, key: &str, expires: u32, params: Vec<(&str, &str)>, oss_config: &OSSConfig) -> String {
    let expires_at = chrono::Utc::now() + chrono::Duration::seconds(expires as i64);
    let expires_at = expires_at.timestamp();
    // 生成签名

    let headers_string = "";
    let sub_resource_string = {
        let s = params.iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<_>>();
        if s.is_empty() {
            "".to_owned()
        } else {
            format!("?{}", s.join("&"))
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
    dbg!(&string_to_sign);
    let mut hmac = hmac::Hmac::<sha1::Sha1>::new_from_slice(oss_config.access_key_secret.as_bytes()).unwrap();
    hmac.update(string_to_sign.as_bytes());
    let signature = base64::encode(&hmac.finalize().into_bytes());


    let params = QueryParams::from(params)
        .add_query_string("OSSAccessKeyId", &oss_config.access_key_id)
        .add_query_string("Expires", &expires_at.to_string())
        .add_query_string("Signature", &signature)
        .stringify();
    if let Some(custom_domain) = &oss_config.custom_domain {
        format!("{}/{}?{}", custom_domain, key, params)
    } else {
        format!("https://{}.{}/{}?{}", oss_config.bucket_name, oss_config.endpoint, key, params)
    }
}