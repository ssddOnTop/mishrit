mod blueprint;
mod from_config;
mod server;
mod try_fold;

use crate::blueprint::try_fold::TryFold;
use crate::config::Config;
pub type TryFoldConfig<'a, A> = TryFold<'a, Config, A, String>;

pub use blueprint::*;
