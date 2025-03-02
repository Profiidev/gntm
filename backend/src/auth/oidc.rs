use rocket::{get, http::CookieJar, response::Redirect, Route, State};

use crate::error::Result;

use super::{jwt::JwtState, state::OIDCState};

pub fn routes() -> Vec<Route> {
  rocket::routes![start_auth, finish_auth]
    .into_iter()
    .flat_map(|route| route.map_base(|base| format!("{}{}", "/oidc", base)))
    .collect()
}

#[get("/start_auth")]
async fn start_auth(state: &State<OIDCState>) -> Result<Redirect> {
  let location = state.start_auth().await?;
  Ok(Redirect::found(location))
}

#[get("/finish_auth?<code>&<state>")]
async fn finish_auth(code: &str, state: &str, app_state: &State<OIDCState>, jwt: &State<JwtState>, jar: &CookieJar<'_>) -> Redirect {
  let info = app_state.finish_auth(code, state).await.unwrap();

  let cookie = jwt.create_token(info.sub).unwrap();
  jar.add(cookie);

  Redirect::found("")
}
