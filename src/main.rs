#![feature(plugin)]
#![feature(rustc_private)]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
extern crate log;

#[macro_use]
extern crate try_opt;
extern crate tokio_core;

mod server;
mod brokers;
mod codecs;
mod config;
mod dashboard;

use dashboard::start_dashboard;
use config::config_manager::ConfigurationManager;

use tokio_core::reactor::Core;

fn main() {
    let mut event_loop = Core::new().unwrap();
    let config_manager = ConfigurationManager::new("athena.config.yml".to_string());
    start_dashboard(config_manager, &mut event_loop);
    //event_loop.run();
}
