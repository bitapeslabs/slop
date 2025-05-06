use crate::p2p::consts;
use clap::Parser;
use serde::Deserialize;
use std::net::SocketAddr;
use std::{fs, str};
use thiserror::Error;
use toml;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Io Error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Deserialize Error {0}")]
    DeserializeError(#[from] toml::de::Error),
}

fn get_default_peers() -> Vec<SocketAddr> {
    vec![consts::SEED_NODE]
}

fn get_default_ip() -> SocketAddr {
    consts::DEFAULT_SOCKET
}

fn get_default_node_config() -> NodeConfig {
    NodeConfig {
        socket: get_default_ip(),
        peers: get_default_peers(),
    }
}

#[derive(Deserialize)]
pub struct NodeConfig {
    /*
        If no peers are provided, and the only peer is ITSELF, the node runs
        as a seed node.
    */
    #[serde(default = "get_default_ip")]
    pub socket: SocketAddr,
    #[serde(default = "get_default_peers")]
    pub peers: Vec<SocketAddr>,
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "get_default_node_config")]
    pub node: NodeConfig,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

pub struct CliArgs {
    /// The pattern to look for
    #[arg(short, long)]
    config: String,
}

pub fn get_config() -> Result<Config, ConfigError> {
    let args = CliArgs::parse();

    let config: String = String::from_utf8_lossy(&fs::read(args.config)?).into_owned();

    Ok(toml::from_str(&config)?)
}
