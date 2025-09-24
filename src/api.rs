use warp::Filter;
use crate::db;

pub async fn run() {
    let get_netflow = warp::path!("netflow").map(|| {
        let rows = db::get_netflows().unwrap_or_default();
        warp::reply::json(&rows)
    });

    println!("API: Listening on http://127.0.0.1:8080");
    warp::serve(get_netflow).run(([127,0,0,1], 8080)).await;
}
