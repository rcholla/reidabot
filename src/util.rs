use crate::prelude::*;

pub struct ReiUtil(pub crate::api::ReiApi);

impl ReiUtil {
  pub async fn is_shadowbanned(&self, username: &str) -> ReiResult<bool> {
    let result = match User::new(username).overview(None).await {
      Ok(_) => false,
      Err(_) => {
        !self
          .0
          .username_available(username)
          .await?
          .json::<bool>()
          .await?
      }
    };

    Ok(result)
  }
}

pub trait ReiFooterExt {
  fn with_footer(&self) -> String;
}

impl<T> ReiFooterExt for T
where
  T: Into<String> + std::fmt::Display,
{
  fn with_footer(&self) -> String {
    f!(
      "
{self}

-----

^({})
      ",
      t!("footer.message")
    )
    .trim()
    .into()
  }
}
