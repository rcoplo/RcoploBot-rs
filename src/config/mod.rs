mod bot_config;
mod config;

use once_cell::sync::Lazy;
pub use bot_config::*;
pub use config::*;

pub static CONFIG_CONTEXT: Lazy<ConfigContext> = Lazy::new(|| ConfigContext::default());

pub struct  ConfigContext {
    pub config: ApplicationConfig,
    pub bot_config: BotConfig,
}

impl Default for ConfigContext{
    fn default() -> Self {
        let config = ApplicationConfig::default();
        let bot_config = BotConfig::default();
        ConfigContext {
            config,
            bot_config,
        }
    }
}
