mod config; // 服务的配置相关内容

#[async_std::main]
async fn main() -> async_std::io::Result<()> {
    let app = tide::new();
    let app = api_basics::init_middlewares(app).await;
    app.listen("0.0.0.0:9000").await
}
