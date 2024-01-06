use lightyear::prelude::Key;

// Use a port of 0 to automatically select a port (bargos: I dont understand this so I bring the comment until I do)
pub const CLIENT_PORT: u16 = 0;
pub const SERVER_PORT: u16 = 5000;
pub const PROTOCOL_ID: u64 = 0;
pub const KEY: Key = [0; 32];
pub mod shared;
pub mod client;
pub mod server;