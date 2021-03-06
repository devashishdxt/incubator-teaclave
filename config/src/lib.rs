#![cfg_attr(feature = "mesalock_sgx", no_std)]
#[cfg(feature = "mesalock_sgx")]
#[macro_use]
extern crate sgx_tstd as std;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ConfigSource {
    Path(PathBuf),
}

mod build;
mod runtime;

pub use build::BUILD_CONFIG;
pub use runtime::RuntimeConfig;
