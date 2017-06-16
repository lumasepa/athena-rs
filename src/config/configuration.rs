extern crate serde_yaml;
extern crate env_logger;

use std::net::Ipv4Addr;
use std::str::FromStr;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use self::serde_yaml::{Value, to_value};
use server::dispatcher::{RouteDescription, Route, RoutingCondition};

use brokers::amqp::AMQPBroker;
use brokers::kafka::{KafkaBroker, KafkaHost};

#[derive(Debug)]
pub struct Configuration {
    pub listen_ip: Ipv4Addr,
    pub listen_port: u32,
    pub routes: Vec<RouteDescription>,
    pub amqp_brokers: Vec<AMQPBroker>,
    pub kafka_brokers: Vec<KafkaBroker>,
}

impl Configuration {

    pub fn from_yaml(yaml_config: Value) -> Configuration {

        Configuration {
            listen_ip: Configuration::parse_listen_ip(&yaml_config),
            listen_port: Configuration::parse_listen_port(&yaml_config),
            routes: Configuration::parse_routes(&yaml_config),
            amqp_brokers: Configuration::parse_amqp_brokers(&yaml_config),
            kafka_brokers: Configuration::parse_kafka_brokers(&yaml_config),
        }
    }

    fn parse_listen_ip(yaml_config: &Value) -> Ipv4Addr {
        yaml_config["listen_ip"]
            .as_str()
            .and_then(|ip|{
                Ipv4Addr::from_str(ip).ok()
            })
            .unwrap_or_else(||{
                warn!("Couldn't parse listen_ip from config file, using 127.0.0.1");
                Ipv4Addr::from_str("127.0.0.1").unwrap()
            })
    }

    fn parse_listen_port(yaml_config: &Value) -> u32 {
        yaml_config["listen_port"]
            .as_u64()
            .map(|port|{
                port as u32
            })
            .unwrap_or_else(||{
                warn!("Couldn't parse listen_port from config file, using 8080");
                8080u32
            })
    }

    fn parse_routes(yaml_config: &Value) -> Vec<RouteDescription> {
        let mut routes: Vec<RouteDescription> = vec![];
        yaml_config["routes"]
            .as_sequence()
            .map(|yaml_routes| {
                yaml_routes
                    .iter()
                    .map(|route|{
                        routes.push(RouteDescription {
                            conditions: Configuration::parse_route_conditions(route),
                            route_to: Configuration::parse_route_to(route),
                        });
                    })
                    .collect::<Vec<_>>();
            });
        routes
        }

    fn parse_amqp_brokers(yaml_config: &Value) -> Vec<AMQPBroker> {
        let mut amqp_brokers: Vec<AMQPBroker> = vec![];
        yaml_config["amqp_brokers"]
            .as_sequence()
            .map(|yaml_amqp_brokers| {
                yaml_amqp_brokers
                    .iter()
                    .map(|amqp_broker|{
                        Configuration::parse_amqp_broker(amqp_broker)
                            .map(|amqp_broker| {
                                amqp_brokers.push(amqp_broker)
                            });
                    })
                    .collect::<Vec<_>>();
            });
        amqp_brokers
    }

    fn parse_kafka_brokers(yaml_config: &Value) -> Vec<KafkaBroker> {

        vec![]
    }

    fn parse_route_conditions(route: &Value) -> Vec<RoutingCondition> {
        let mut conditions: Vec<RoutingCondition> = vec![];
        route["conditions"]
            .as_sequence()
            .map(|yaml_conditions| {
                yaml_conditions
                    .iter()
                    .map(|condition|{
                        let field: String = match condition["field"].as_str() {
                            None => return,
                            Some(field) => field.to_string(),
                        };
                        let value: String = match condition["value"].as_str() {
                            None => return,
                            Some(value) => value.to_string(),
                        };
                        conditions.push(RoutingCondition {
                                            field: field,
                                            value: value,
                                        });
                    })
                    .collect::<Vec<_>>();
            });
        conditions
    }

    fn parse_route_to(route: &Value) -> Vec<Route> {
        let mut route_to: Vec<Route> = vec![];
        route["route_to"]
            .as_sequence()
            .map(|yaml_route_to|{
                yaml_route_to
                    .iter()
                    .map(|route| {
                        let broker = match route["broker"].as_str() {
                            None => return,
                            Some(broker) => broker.to_string(),
                        };
                        let queue = match route["queue"].as_str() {
                            None => return,
                            Some(queue) => queue.to_string(),
                        };
                        route_to.push(Route {
                                          broker: broker,
                                          queue: queue,
                                      });
                    })
                    .collect::<Vec<_>>();
            });
        route_to
    }

    fn parse_amqp_broker(amqp_broker: &Value) -> Option<AMQPBroker> {
        let name = try_opt!(amqp_broker["name"].as_str()).to_string();
        let host = try_opt!(amqp_broker["host"].as_str()).to_string();

        let port = amqp_broker["port"].as_i64().unwrap_or(5672i64);
        let user = try_opt!(amqp_broker["user"].as_str()).to_string();
        let password = try_opt!(amqp_broker["password"].as_str()).to_string();

        let vhost = amqp_broker["vhost"].as_str().unwrap_or_else(||{
            warn!("vhost of  broker {} not found in config using /", name);
            "/"
        }).to_string();

        Some(
            AMQPBroker{
                host: host,
                port: port as u32,
                user: user,
                password: password,
                vhost: vhost,
                name: name,
        })
    }
}


mod test_configuration_parse_from_yaml {

    #[test]
    fn it_should_parse_the_yaml_file_in_the_configuration_struct() {
        use super::super::config_manager::ConfigurationManager;
        use super::Configuration;

        let config_file = ConfigurationManager::load_config_file(&"athena.config.yml".to_string());
        let yaml_config = ConfigurationManager::deserialize_config_file(config_file);
        let config = Configuration::from_yaml(yaml_config);

        println!("{:#?}", config);

        assert_eq!(config.listen_port, 8080u32);
        assert_eq!(config.routes.len(), 2);
        assert_eq!(config.amqp_brokers.len(), 1);
    }
}
