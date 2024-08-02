#![allow(unused)]
#![allow(clippy::module_inception)]
mod blueprint;
mod config;
mod http;
mod plan;
mod runtime;
pub mod tracing;
mod valid;

pub(crate) fn is_default<T: Default + PartialEq>(val: &T) -> bool {
    T::default().eq(val)
}
