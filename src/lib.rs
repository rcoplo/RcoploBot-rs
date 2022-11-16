#![allow(unused_variables)] //允许未使用的变量
#![allow(unused_must_use)]

#[macro_use]
extern crate rbatis;

#[macro_use]
pub mod handler;
pub mod core;
pub mod config;
pub mod api;
pub mod util;
pub mod domain;
pub mod service;
