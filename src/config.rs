use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
  pub username: String,
  pub password: String,
  pub client_id: String,
  pub client_secret: String,
  pub user_agent: String,
}

#[derive(strum::AsRefStr)]
pub enum Locale {
  TR,
  EN,
}
