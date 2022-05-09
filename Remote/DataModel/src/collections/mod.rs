pub mod files;

use mongodm::{Repository, Model};

#[async_trait::async_trait]
pub trait ById: Model + Sync {
    async fn by_id<T: Into<String> + Send>(repo: &Repository<Self>, id: T) -> Option<Self> {
        // 默认实现
        let result = repo.find_one(bson::doc! {
            "id": id.into()
        }, None).await;
        if let Ok(option) = result {
            if let Some(doc) = option {
                return Some(doc);
            }
        }
        None
    }
}