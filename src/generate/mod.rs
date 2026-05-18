mod nu;
mod pwsh;
mod shared;
mod zsh;

use anyhow::Result;

use crate::config::Config;

pub struct Generator<'a> {
    config: &'a Config,
}

impl<'a> Generator<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    pub fn zsh(&self) -> Result<String> {
        zsh::generate(self.config)
    }

    pub fn nu(&self) -> Result<String> {
        nu::generate(self.config)
    }

    pub fn pwsh(&self) -> Result<String> {
        pwsh::generate(self.config)
    }
}
