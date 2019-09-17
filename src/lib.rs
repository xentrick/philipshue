#[macro_use]
extern crate serde_derive;
#[cfg(feature = "ssdp")]
extern crate ssdp;

#[cfg(feature = "nupnp")]
extern crate hyper_tls;
#[macro_use]
extern crate error_chain;

pub use crate::bridge::Bridge;
#[cfg(feature = "nupnp")]
pub use crate::bridge::discover;
#[cfg(feature = "upnp")]
pub use crate::bridge::discover_upnp;
pub use crate::hue::LightCommand;

/// Errors that can occur in the crate
pub mod errors;
/// Handles all the communication with the bridge
pub mod bridge;
/// Structs mapping the different JSON-objects used with Hue API
pub mod hue;
mod json;
