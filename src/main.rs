use axum::{
    http::{StatusCode},
    response::{Html, IntoResponse},
    routing::{get},
    Router,
};
use std::net::SocketAddr;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};

mod login;

use crate::login::{login_form, login_post};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/login", get(login_form).post(login_post))
        .route("/logout", get(logout));

    // layers (includes our cookie library)
    let app = app.layer(CookieManagerLayer::new());

    // fallback (404s)
    let app = app.fallback(get(handler_404));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root(c: Cookies) -> impl IntoResponse {
    let logged_in = c
        .get("kagi_auth_98b3")
        .and_then(|k| Some(k.value().to_string()))
        .unwrap_or("false".to_string());

    if logged_in == "true" {
        (StatusCode::IM_A_TEAPOT, Html("<code>welcome to Super Hyper Ultra Ultimate Deluxe Perfect Amazing Shining God 東方不敗 Master Ginga Victory Strong Cute Beautiful Galaxy Baby 無限 無敵 無双 kagi microsystems inc. u are loged in. <a href=\"/logout\">log out.</a> </code>"))
    } else {
        (StatusCode::OK, Html("<code>Hello World.</code>"))
    }
}

async fn logout(c: Cookies) -> impl IntoResponse {
    c.remove(Cookie::new("kagi_auth_98b3", ""));
    Html("<code>you have been logged out.</code><script>let r = () =>window.location = \"/\"\nsetTimeout(r, \"500\")</script>")
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Html("<img src=\"https://http.cat/404\">"),
    )
}

#[cfg(unix)]
pub async fn shutdown_signal() {
    use std::io;
    use tokio::signal::unix::SignalKind;

    async fn terminate() -> io::Result<()> {
        tokio::signal::unix::signal(SignalKind::terminate())?
            .recv()
            .await;
        Ok(())
    }

    tokio::select! {
        _ = terminate() => {},
        _ = tokio::signal::ctrl_c() => {},
    }
    println!("signal received, starting graceful shutdown")
}

#[cfg(windows)]
pub async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("faild to install CTRL+C handler");
    println!("signal received, starting graceful shutdown")
}
