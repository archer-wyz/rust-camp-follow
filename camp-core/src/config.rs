use thiserror::Error;
#[derive(Debug, Error)]
pub enum ConfigErr {
    #[error("Open config error: {0}")]
    NotExists(String),

    #[error("Parse config error: {0}")]
    Parse(#[from] serde_yaml::Error),
}

pub fn config_load<T>(paths: Vec<String>) -> Result<T, ConfigErr>
where
    T: serde::de::DeserializeOwned,
{
    for path in &paths {
        match std::fs::File::open(path) {
            Ok(file) => {
                let config: T = serde_yaml::from_reader(file)?;
                return Ok(config);
            }
            Err(_) => continue,
        }
    }
    let path_string = format!("{:?} not exists", paths);
    Err(ConfigErr::NotExists(path_string))
}
