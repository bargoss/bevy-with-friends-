use std::net::IpAddr;
use bevy::prelude::Resource;

#[derive(PartialEq, Resource, Copy, Clone)]
pub enum NetworkConfiguration {
    Server {
        port: u16,
    },
    Client {
        ip: IpAddr,
        port: u16,
    },
}