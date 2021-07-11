use warp::Filter;

#[tokio::main]
async fn main() {
    let hi = warp::path("hello")
        .and(warp::path::param())
        .and(warp::header("user-agent"))
        .map(|param: String, agent: String| {
            format!("Hello {}, whose agent is {}", param, agent)
        });

    // 注册全部路由
    let route = hi;

    // 启动web
    warp::serve(route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
