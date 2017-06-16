extern crate serde_yaml;
extern crate env_logger;

use std::net::Ipv4Addr;
use std::str::FromStr;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use self::serde_yaml::{Value, to_value};
use super::configuration::Configuration;


pub struct ConfigurationManager {
    configuration: Configuration,
    config_file_path: String
}

impl ConfigurationManager {

    pub fn new(config_file_path: String) -> ConfigurationManager{
        ConfigurationManager {
            configuration: ConfigurationManager::load_config(&config_file_path),
            config_file_path: config_file_path,
        }
    }

    pub fn load_config_file(config_file_path: &String) -> String {
        let file_path = Path::new(&config_file_path);

        let mut file = File::open(&file_path).unwrap_or_else(|e| {
            panic!("Couldn't open config file {}: {}", config_file_path, e.description());
        });

        let mut file_content = String::new();

        file.read_to_string(&mut file_content).unwrap_or_else(|e|{
            panic!("Couldn't read configfile {}: {}", config_file_path, e.description());
        });
        file_content
    }

    pub fn deserialize_config_file(config_file: String) -> Value {
        serde_yaml::from_str(&config_file).unwrap_or_else(|e|{
            panic!("Couldn't parse config file : {}", e.description());
        })
    }

    pub fn load_config(config_path: &String) -> Configuration {
        let config_file = ConfigurationManager::load_config_file(config_path);
        let yaml_config = ConfigurationManager::deserialize_config_file(config_file);
        Configuration::from_yaml(yaml_config)
    }
}


mod test_configuration_load {
    use super::ConfigurationManager;

    #[test]
    fn it_should_load_the_yaml_file() {
        let yaml_content = ConfigurationManager::load_config_file(&"athena.config.yml".to_string());
        assert!(yaml_content.contains("rabbitmq1"));
    }

    #[test]
    fn it_should_deserialize_the_yaml_file() {
        let yaml_file = ConfigurationManager::load_config_file(&"athena.config.yml".to_string());
        let yaml_config = ConfigurationManager::deserialize_config_file(yaml_file);
        assert_eq!(yaml_config["listen_ip"].as_str().unwrap(), "127.0.0.1");
    }

    #[test]
    fn it_should_parse_the_yaml_file_in_the_configuration_struct() {
        let config_manager = ConfigurationManager::new("athena.config.yml".to_string());

        println!("{:#?}", config_manager.configuration);

        assert_eq!(config_manager.configuration.listen_port, 8080u32);
        assert_eq!(config_manager.configuration.routes.len(), 2);
        assert_eq!(config_manager.configuration.amqp_brokers.len(), 1);
    }
}