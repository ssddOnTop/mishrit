use crate::blueprint::server::Server;
use crate::blueprint::try_fold::TryFold;
use crate::blueprint::{Blueprint, TryFoldConfig};
use crate::config::Config;
use crate::valid::{Valid, Validator};

pub fn config_blueprint<'a>() -> TryFold<'a, Config, Blueprint, String> {
    let server = TryFoldConfig::<Blueprint>::new(|config, blueprint| {
        Valid::from(Server::try_from(&config.server)).map(|v| blueprint.server(v))
    });
    server
}
