use reidabot::prelude::*;

#[tokio::main]
async fn main() -> ReiResult {
  Rei::init_tracing()?;

  let env = Rei::load_env()?;
  let _rei = Rei::new(env).await?;

  Ok(())
}
