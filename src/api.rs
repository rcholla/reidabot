use crate::prelude::*;

#[derive(Clone)]
pub struct ReiApi(pub reqwest::Client);

impl ReiApi {
  pub async fn remove(&self, target: &str, spam: bool) -> ReiResult<reqwest::Response> {
    let response = self
      .0
      .post(build_oauth("api/remove"))
      .query(&[("id", target), ("spam", &spam.to_string())])
      .send()
      .await?;

    Ok(response)
  }

  pub async fn username_available(&self, username: &str) -> ReiResult<reqwest::Response> {
    let response = self
      .0
      .post(build_oauth("api/username_available"))
      .query(&[("user", username)])
      .send()
      .await?;

    Ok(response)
  }
}
