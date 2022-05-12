use std::string::ToString;
use mongodm::prelude::*;
use serde::{Deserialize, Serialize};


pub struct CollConf;

impl CollectionConfig for CollConf {
    fn collection_name() -> &'static str {
        "..." // collection name
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
pub struct MyModel {
    pub id: uuid::Uuid,
    // ...
}

impl Model for MyModel {
    type CollConf = CollConf;
}


#[cfg(test)]
mod tests {
    use super::*;
    #[async_std::test]
    fn it_works() {
        // ...
    }
}
