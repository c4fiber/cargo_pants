use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

fn main() {
    // || { App::new() ...}는 러스트 클로저(closure) 표현식임. 함수처럼 호출할 수 있는 값
    // || 사이에 인수의 이름이 와야함, 없으면 비워둠
    let server = HttpServer::new(|| {

        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Server on http://localhost:3000 ... ");
    server
        .bind("127.0.0.1:3000").expect("error binding server to address")
        .run().expect("error running server");
}

fn get_index() -> HttpResponse {
    // 요청이 성공했음을 알리는 HTTP 200 OK를 표현한다.
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute GCD</button>
                </form>
            "#,
        )
    // r#" "#  (# 대신 해시기호로 대체가능, 갯수를 맞추기만 하면 됨)
    // rust 원시 문자열문법이며 따옴표 내부의 모든 텍스트는 이스케이프 처리없이 온다.
}
// 역직렬화(Deserialize)를 가능하게 해서 요청핸들러는 web:Form(GcdParameters>값을 매개변수로 받을 수 있게 한다.
#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}
// 액틱스 요청 핸들러로 쓸 함수의 모든 인수는 액틱스가 HTTP 요청에서 빼낼 수 있는 타입이어야 한다.
fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }

    let response = format!("The greatest common divisor of the numbers {} and {} \
                            is <b>{}</b>\n",
                            form.n, form.m, gcd(form.n, form.m));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t: u64 = m; // 지역변수는 let으로 선언한다. 반복문이 수행될 때 마다 선언되므로 mut 불필요
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                   3 * 7 * 11 * 13 * 19),
               3 * 11);
}