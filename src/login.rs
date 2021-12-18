use axum::{
    extract::Form,
    http::{header::SET_COOKIE, HeaderMap},
    response::{Html, IntoResponse},
};
use serde::Deserialize;

pub async fn login_form() -> Html<&'static str> {
    Html(
        r#"
                <form action="/login" method="post">
                    <label for="username">
                        <input type="text" name="username">
                    </label>
                    <label>
                        <input type="text" name="password">
                    </label>
                    <input type="submit" value="go!">
                </form>
        "#,
    )
}
#[derive(Deserialize, Debug)]
pub struct UserLogin {
    username: String,
    password: String,
}

pub async fn login_post(Form(input): Form<UserLogin>) -> impl IntoResponse {
    let cookie = "kagi_auth_98b3=true; SameSite=Lax; Path=/";
    let mut headers = HeaderMap::new();

    if input.username == "miwa" && input.password == "hunter2" {
        headers.insert(SET_COOKIE, cookie.parse().unwrap());

        (headers, Html("<code>you have been logged in.</code><script>let r = () =>window.location = \"/\"\nsetTimeout(r, \"500\")</script>"))
    } else {
        (headers, Html("<code>Wrong username or password.</code>"))
    }
}
