#![allow(unused)]
#![allow(clippy::module_inception)]
mod config;
mod http;
mod plan;

pub fn is_default<T: Default + PartialEq>(val: &T) -> bool {
    T::default().eq(val)
}
