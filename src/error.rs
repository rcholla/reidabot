use crate::prelude::*;
use serde::{Deserialize, Serialize};
use strum::EnumMessage;
use tracing_error::SpanTrace;

pub type ReiResult<T = (), E = ReiError> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct ReiError {
  pub error_type: ReiErrorType,
  pub inner: anyhow::Error,
  pub context: SpanTrace,
}

impl std::fmt::Display for ReiError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}: ", self.error_type)?;
    writeln!(f, "{:?}", self.inner)?;
    std::fmt::Display::fmt(&self.context, f)
  }
}

impl<T> From<T> for ReiError
where
  T: Into<anyhow::Error>,
{
  fn from(err: T) -> Self {
    let inner = err.into();

    Self {
      error_type: ReiErrorType::Unknown(f!("{inner}")),
      inner,
      context: SpanTrace::capture(),
    }
  }
}

#[rustfmt::skip]
#[derive(Debug, strum::Display, EnumMessage, Serialize, Deserialize)]
pub enum ReiErrorType {
  #[strum(message = "Unable to load environment variables")] LoadEnvVariables,
  #[strum(message = "Unable to subscribe tracing")] SubscribeTracing,
  #[strum(message = "Unable to create a reddit instance")] CreateInstance,
  Unknown(String),
}

impl ReiErrorType {
  fn as_anyhow(&self) -> anyhow::Error {
    match &self {
      ReiErrorType::Unknown(reason) => anyhow!("{reason}"),
      _ => anyhow!("{}", self.get_message().unwrap_or("None")),
    }
  }
}

impl From<ReiErrorType> for ReiError {
  fn from(error_type: ReiErrorType) -> Self {
    let inner = error_type.as_anyhow();

    Self {
      error_type,
      inner,
      context: SpanTrace::capture(),
    }
  }
}

pub trait ReiErrorExt<T, E> {
  fn yeets(self, error_type: ReiErrorType) -> ReiResult<T>;
}

impl<T, E> ReiErrorExt<T, E> for std::result::Result<T, E>
where
  E: Into<anyhow::Error>,
{
  fn yeets(self, error_type: ReiErrorType) -> ReiResult<T> {
    let inner = error_type.as_anyhow();

    self.map_err(|_| ReiError {
      error_type,
      inner,
      context: SpanTrace::capture(),
    })
  }
}

impl<T, E> ReiErrorExt<T, E> for ReiResult<T> {
  fn yeets(self, error_type: ReiErrorType) -> ReiResult<T> {
    self.map_err(|mut err| {
      err.error_type = error_type;
      err
    })
  }
}
