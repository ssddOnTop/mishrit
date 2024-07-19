#![allow(unused)]
#![allow(clippy::module_inception)]
mod blueprint;
mod config;
mod http;
mod plan;
mod runtime;
mod valid;

pub fn is_default<T: Default + PartialEq>(val: &T) -> bool {
    T::default().eq(val)
}
