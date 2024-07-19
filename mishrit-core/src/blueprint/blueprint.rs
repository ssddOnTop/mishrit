#![allow(clippy::needless_update)]

use crate::blueprint::server::Server;

#[derive(Clone, derive_setters::Setters)]
pub struct Blueprint {
    pub server: Server,
}
