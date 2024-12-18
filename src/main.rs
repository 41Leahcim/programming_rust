use actix_web::{App, HttpResponse, HttpServer, web};

pub const fn gcd(mut left: u64, mut right: u64) -> u64 {
    assert!(left != 0 && right != 0);
    while right != 0 {
        if right < left {
            (left, right) = (right, left);
        }
        right %= left;
    }
    left
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"<title>GCD calculator</title>
    <form action="/gcd" method="post">
    <input type="number" name="n">
    <input type="number" name="m">
    <button type="submit">Compute GCD</button>
    </form>"#,
    )
}

#[actix_web::main]
async fn main() {
    println!("Serving on http://localhost:3000");
    HttpServer::new(|| App::new().route("/", web::get().to(get_index)))
        .bind("127.0.0.1:3000")
        .expect("Failed to bind server to address")
        .run()
        .await
        .expect("Failed to run server");
}
