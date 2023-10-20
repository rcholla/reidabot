#![feature(yeet_expr)]

mod api;
mod env;
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
  pub fn load_env() -> ReiResult<EnvVariables> {
    tracing::info!("Loading environment variables");
    dotenvy::dotenv().yeets(ReiErrorType::LoadEnvVariables)?;
    let env = envy::from_env::<EnvVariables>().yeets(ReiErrorType::LoadEnvVariables)?;

    Ok(env)
  }

  pub fn init_tracing() -> ReiResult {
    let subscriber = FmtSubscriber::builder()
      .with_max_level(tracing::Level::INFO)
      .finish();
    tracing::subscriber::set_global_default(subscriber).yeets(ReiErrorType::SubscribeTracing)?;

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

  pub async fn new(env: EnvVariables) -> ReiResult<Self> {
    tracing::info!("Rei is getting ready to fly to the moon!");
    let me = Reddit::new(&env.user_agent, &env.client_id, &env.client_secret)
      .username(&env.username)
      .password(&env.password)
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
