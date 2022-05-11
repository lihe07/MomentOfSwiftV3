pub mod files;

use std::time::Duration;
use r_cache::cache::Cache;
use mongodm::Model;
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
/// 实现了这个trait的模型可以被缓存，并以id查找
pub trait ById: Model + Sync {
    /// 根据id获取一个实例
    /// 会优先从缓存中获取，如果缓存中没有，则从数据库中获取（并加入缓存）
    async fn by_id<T: Into<String> + Send>(id: T) -> Option<Self>;
    /// 根据多个ID获取多个实例
    async fn by_ids<T: Into<Vec<String>> + Send>(ids: T) -> Vec<Self>;
    /// 将自身同步到缓存
    async fn sync(&self);
}

#[async_trait::async_trait]
/// 实现了这个trait的模型可以转换为json返回给前端
pub trait ToJson: serde::Serialize {
    /// 转换为json
    /// 会处理 #[hidden] 和 #[expand(...)] 标签
    /// 会转换bson::DateTime为时间戳
    async fn to_json(self) -> serde_json::Value;
    /// 忽略所有的expand属性，其他特性与to_json相同
    fn to_json_without_expand(self) -> serde_json::Value;
}