use log::info;

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, Clone)]
pub struct ApplicationConfig {
    pub debug: bool,
    pub database_url: String,
}
impl Default for ApplicationConfig {
    fn default() -> Self {
        let yml_data = include_str!("../../config/application.yml");
        let result: ApplicationConfig =
            serde_yaml::from_str(yml_data).expect("[RcoploBot] load config file fail");
        if result.debug {
            info!("[RcoploBot] load config:{:?}", result);
            info!("[RcoploBot] ///////////////////// Start On Debug Mode ////////////////////////////");
        } else {
            info!("[RcoploBot] ///////////////////// Start On Release Mode ////////////////////////////");
        }
        result
    }
}