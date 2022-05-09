pub mod files;

use std::time::Duration;
use r_cache::cache::Cache;
use mongodm::{Repository, Model};
use async_std::task;
use async_std::sync::Arc;

lazy_static::lazy_static! {
    static ref CACHE: Arc<Cache<String, Vec<u8>>> = {
        let cache = Arc::new(Cache::new(Some(Duration::from_secs(crate::config::CACHE_TTL))));
        task::spawn({
            let cache = cache.clone();
            async move {
                loop {
                    task::sleep(Duration::from_secs(crate::config::CACHE_TTL)).await;
                    cache.remove_expired().await;
                }
            }
        });
        cache
    };
}

#[async_trait::async_trait]
pub trait ById: Model + Sync {
    async fn by_id<T: Into<String> + Send>(repo: &Repository<Self>, id: T) -> Option<Self> {
        // 默认实现
        // 先从缓存中查找
        let id = id.into();
        if let Some(data) = CACHE.get(&id).await {
            Some(bson::from_slice(data.as_slice()).unwrap())
        } else {
            // 如果缓存中没有，则从数据库中查找
            let result = repo.find_one(bson::doc! {
                "id": id.to_owned()
            }, None).await;
            if let Ok(option) = result {
                if let Some(doc) = option {
                    // 将查找到的数据放入缓存
                    CACHE.set(id, bson::to_vec(&doc).unwrap(), None).await;
                    return Some(doc);
                }
            }
            None
        }
    }
    async fn by_ids<T: Into<Vec<String>> + Send>(repo: &Repository<Self>, ids: T) -> Vec<Self> {
        // 默认实现 循环调用 by_id
        let ids = ids.into();
        let mut result = Vec::new();
        for id in ids {
            if let Some(doc) = Self::by_id(repo, id).await {
                result.push(doc);
            }
        }
        result
    }
}