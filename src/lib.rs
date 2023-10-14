mod env;
mod error;
pub mod prelude;

use crate::{env::EnvVariables, prelude::*};
use roux::Reddit;
use tracing_subscriber::FmtSubscriber;

pub struct Rei {
  pub me: roux::Me,
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
}

impl Rei {
  pub async fn new(env: EnvVariables) -> ReiResult<Self> {
    tracing::info!("Rei is getting ready to fly to the moon!");
    let me = Reddit::new(&env.user_agent, &env.client_id, &env.client_secret)
      .username(&env.username)
      .password(&env.password)
      .login()
      .await
      .yeets(ReiErrorType::CreateInstance)?;

    Ok(Self { me })
  }
}
