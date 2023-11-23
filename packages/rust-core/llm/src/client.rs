use async_trait::async_trait;

use crate::{
  error::{ClientCreationError, ExecutorError},
  model::{Output, Prompt},
  options::{BuiltinOptions, Options},
};

#[async_trait]
pub trait Client: Sized {
  type Opt<'a>: Default
  where
    Self: 'a;

  fn new_with_options(options: Options<Self::Opt<'_>>) -> Result<Self, ClientCreationError>;

  fn new_with_custom_options(custom: Self::Opt<'_>) -> Result<Self, ClientCreationError> {
    Self::new_with_options(Options::new(BuiltinOptions::default(), custom))
  }

  fn new() -> Result<Self, ClientCreationError> {
    Self::new_with_options(Default::default())
  }

  async fn chat_complete(
    &self,
    prompt: Prompt,
    config: BuiltinOptions,
  ) -> Result<Output, ExecutorError>;
}