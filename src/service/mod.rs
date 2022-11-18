use log::info;
use once_cell::sync::Lazy;
use rbatis::Rbatis;
use rbdc_mysql::driver::MysqlDriver;
use crate::config::ApplicationConfig;
use crate::config::BotConfig;

mod setu_service;
mod friend_function_service;
mod group_function_service;
mod sign_group_users_service;

pub use setu_service::*;
pub use group_function_service::*;
pub use friend_function_service::*;
pub use sign_group_users_service::*;

/// CONTEXT is all of the service struct
pub static CONTEXT: Lazy<ServiceContext> = Lazy::new(|| ServiceContext::default());

#[macro_export]
macro_rules! pool {
    () => {
        &mut $crate::service::CONTEXT.rbatis.clone()
    };
}

pub struct  ServiceContext {
    pub config: ApplicationConfig,
    pub bot_config: BotConfig,
    pub rbatis :Rbatis,
}

impl ServiceContext {
    /// init database pool
    pub async fn init_pool(&self) {
        //连接数据库
        info!(
            "[RcoploBot] rbatis pool init ({})...",
            self.config.database_url
        );
        self.rbatis
            .init(MysqlDriver {}, &self.config.database_url)
            .expect("[abs_admin] rbatis pool init fail!");
        log::info!(
            "[RcoploBot] rbatis pool init success! pool state = {:?}",
            self.rbatis.get_pool().expect("pool not init!").status()
        );
        log::info!(
            " - Local:   http://{}",
            self.config.database_url.replace("0.0.0.0", "127.0.0.1")
        );
    }

}
impl Default for ServiceContext{
    fn default() -> Self {
        let config = ApplicationConfig::default();
        let bot_config = BotConfig::default();
        ServiceContext {
            rbatis:crate::domain::init_rbatis(&config),
            config,
            bot_config,
        }
    }
}

