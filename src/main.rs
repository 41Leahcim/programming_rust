use actix_web::{App, HttpResponse, HttpServer, web};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GcdParameters {
    left: u64,
    right: u64,
}

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
    <input type="number" name="left">
    <input type="number" name="right">
    <button type="submit">Compute GCD</button>
    </form>"#,
    )
}

async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.left == 0 || form.right == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is impossible");
    }
    HttpResponse::Ok().content_type("text/html").body(format!(
        "The gratest common divisor of the numbers {} and {} is <b>{}</b>",
        form.left,
        form.right,
        gcd(form.left, form.right)
    ))
}

#[actix_web::main]
async fn main() {
    println!("Serving on http://localhost:3000");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    })
    .bind("127.0.0.1:3000")
    .expect("Failed to bind server to address")
    .run()
    .await
    .expect("Failed to run server");
}
