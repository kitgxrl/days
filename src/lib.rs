#![feature(path_try_exists)]
#![allow(unused)]

#[macro_use]
extern crate log;

mod error;
mod index;
mod package;
mod store;
#[cfg(test)]
mod tests;

pub fn log_in_lib() {
    debug!("Hello world!")
}
