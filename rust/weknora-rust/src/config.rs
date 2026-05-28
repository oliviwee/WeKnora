use std::{env, net::IpAddr, str::FromStr};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub server: ServerConfig,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ServerConfig {
    pub host: IpAddr,
    pub port: u16,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ConfigError {
    InvalidHost { value: String, message: String },
    InvalidPort { value: String, message: String },
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidHost { value, message } => {
                write!(f, "invalid WEKNORA_SERVER_HOST {value:?}: {message}")
            }
            Self::InvalidPort { value, message } => {
                write!(f, "invalid WEKNORA_SERVER_PORT {value:?}: {message}")
            }
        }
    }
}

impl std::error::Error for ConfigError {}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let host =
            parse_host(env::var("WEKNORA_SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".into()))?;
        let port = parse_port(env::var("WEKNORA_SERVER_PORT").unwrap_or_else(|_| "8080".into()))?;

        Ok(Self {
            server: ServerConfig { host, port },
        })
    }

    pub fn for_tests() -> Self {
        Self {
            server: ServerConfig {
                host: IpAddr::from([127, 0, 0, 1]),
                port: 0,
            },
        }
    }

    pub fn server_addr(&self) -> std::net::SocketAddr {
        std::net::SocketAddr::new(self.server.host, self.server.port)
    }
}

fn parse_host(value: String) -> Result<IpAddr, ConfigError> {
    IpAddr::from_str(&value).map_err(|err| ConfigError::InvalidHost {
        value,
        message: err.to_string(),
    })
}

fn parse_port(value: String) -> Result<u16, ConfigError> {
    value
        .parse::<u16>()
        .map_err(|err| ConfigError::InvalidPort {
            value,
            message: err.to_string(),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_for_tests_is_loopback_with_ephemeral_port() {
        let config = Config::for_tests();
        assert_eq!(config.server.host, IpAddr::from([127, 0, 0, 1]));
        assert_eq!(config.server.port, 0);
    }
}
