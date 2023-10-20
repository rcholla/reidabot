use reidabot::prelude::*;

i18n!("locales", fallback = "EN");

#[tokio::main]
async fn main() -> ReiResult {
  Rei::init_tracing(tracing::Level::INFO)?;

  Rei::set_locale("TR")?;

  let env = Rei::load_env()?;
  let rei = Rei::new(env).await?;

  Rei::stream_posts("CodingTR", |post| warn_shadowbanned_users(&rei, post)).await?;

  Ok(())
}

async fn warn_shadowbanned_users(rei: &Rei, post: SubmissionData) -> ReiResult {
  if post.author != "[deleted]" && rei.util.is_shadowbanned(&post.author).await? {
    let message = t!("warning.shadowban", author = post.author).with_footer();
    rei.me.comment(&message, &post.name).await?;

    rei.api.remove(&post.name, false).await?;
  }

  Ok(())
}
