use warp::Filter;

#[tokio::main]
async fn main() {
    // 注册filter
    let hello_path_echo = warp::any()
        .and(warp::path!("hello_pa"/ String))
        .map(|str:String| format!("hello {}", str));

    let hello = warp::any()
        .and(warp::path("hello"))
        .map(||format!("hello"));

    // 注册全部路由
    let route = hello.or(hello_path_echo);

    // 启动web
    warp::serve(route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
