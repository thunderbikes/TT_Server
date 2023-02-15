use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));
    let version = warp::path!("version")
        .map(|| format!("Version: 0.1.0"));
    let goodbye = warp::path!("goodbye" / String)
        .map(|name: String| format!("bye {}!", name));
    warp::serve(hello.or(version).or(goodbye))
        .run(([127, 0, 0, 1], 3030))
        .await;
    
}