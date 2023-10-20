#![feature(yeet_expr)]

mod api;
mod config;
mod error;
pub mod prelude;
mod util;

use crate::prelude::*;
use futures::{Future, StreamExt};
use std::{iter::Take, time::Duration};
use tokio_retry::strategy::ExponentialBackoff;
use tracing_subscriber::FmtSubscriber;

i18n!("locales", fallback = "EN");

pub struct Rei {
  pub me: roux::Me,
  pub api: api::ReiApi,
  pub util: util::ReiUtil,
}

impl Rei {
  pub fn load_config() -> ReiResult<Config> {
    tracing::info!("Loading config");
    dotenvy::dotenv().yeets(ReiErrorType::LoadConfig)?;
    let config = envy::from_env::<Config>().yeets(ReiErrorType::LoadConfig)?;

    Ok(config)
  }

  pub fn init_tracing(level: tracing::Level) -> ReiResult {
    tracing::subscriber::set_global_default(
      FmtSubscriber::builder().with_max_level(level).finish(),
    )
    .yeets(ReiErrorType::SubscribeTracing)?;

    Ok(())
  }

  pub fn set_locale(locale: &str) -> ReiResult {
    if !available_locales!().contains(&locale) {
      do yeet ReiErrorType::SetLocale(locale.into());
    }

    tracing::info!("Setting locale to '{locale}'");
    rust_i18n::set_locale(locale);

    Ok(())
  }

  pub async fn new(config: Config) -> ReiResult<Self> {
    #[rustfmt::skip] let Config { username, password, client_id, client_secret, user_agent } = config;

    tracing::info!("Rei is getting ready to fly to the moon!");
    let me = Reddit::new(&user_agent, &client_id, &client_secret)
      .username(&username)
      .password(&password)
      .login()
      .await
      .yeets(ReiErrorType::CreateInstance)?;

    let rei_api = api::ReiApi(me.client.clone()); // NOTE: i hate this, but it works...

    Ok(Self {
      me,
      api: rei_api.clone(),
      util: util::ReiUtil(rei_api),
    })
  }
}

type StreamOptions = (
  Subreddit,
  Duration,
  Take<ExponentialBackoff>,
  Option<Duration>,
);

impl Rei {
  fn stream_options(subreddit: &str) -> StreamOptions {
    (
      Subreddit::new(subreddit),
      Duration::from_secs(60),
      ExponentialBackoff::from_millis(5).factor(100).take(3),
      Some(Duration::from_secs(10)),
    )
  }

  pub async fn stream_posts<F, R>(subreddit: &str, cb: F) -> ReiResult
  where
    F: Fn(SubmissionData) -> R,
    R: Future<Output = ReiResult>,
  {
    let opts = Self::stream_options(subreddit);
    let (mut stream, join_handle) =
      roux_stream::stream_submissions(&opts.0, opts.1, opts.2, opts.3);

    while let Some(post) = stream.next().await {
      cb(post?).await?;
    }

    join_handle.await??;

    Ok(())
  }

  pub async fn stream_comments<F, R>(subreddit: &str, cb: F) -> ReiResult
  where
    F: Fn(CommentData) -> R,
    R: Future<Output = ReiResult>,
  {
    let opts = Self::stream_options(subreddit);
    let (mut stream, join_handle) = roux_stream::stream_comments(&opts.0, opts.1, opts.2, opts.3);

    while let Some(comment) = stream.next().await {
      cb(comment?).await?;
    }

    join_handle.await??;

    Ok(())
  }
}
