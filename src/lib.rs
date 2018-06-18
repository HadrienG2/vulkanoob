//! A shameless collection of ugly conveniences for vulkano-based code
//!
//! This library provides shortcuts to ease usage of the vulkano library in
//! quick application prototypes. It should not be used in production code.

#[macro_use] extern crate failure;
#[macro_use] extern crate log;

extern crate vulkano;

pub mod instance;
pub mod device;

use std::result;


/// We use failure's type-erased error handling
pub type Result<T> = result::Result<T, failure::Error>;