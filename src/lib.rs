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

use vulkano::{
    device::DeviceExtensions,
    instance::{
        Features,
        PhysicalDevice,
        QueueFamily,
        Version,
    }
};


/// We use failure's type-erased error handling
pub type Result<T> = result::Result<T, failure::Error>;


/// Helper for building vulkanoob device filters
///
/// Features all the basic device selection criteria which you will almost
/// always want to specify when using vulkanoob.
///
pub fn easy_device_filter<'a>(
    features: &'a Features,
    extensions: &'a DeviceExtensions,
    queue_filter: &'a mut (impl FnMut(&QueueFamily) -> bool + 'a),
    mut other_criteria: impl FnMut(PhysicalDevice) -> bool + 'a
) -> impl FnMut(PhysicalDevice) -> bool + 'a {
    move |dev: PhysicalDevice| -> bool {
        // This library was written against Vulkan v1.0.76. We tolerate older
        // patch releases and new minor versions but not new major versions.
        let min_ver = Version { major: 1, minor: 0, patch: 0 };
        let max_ver = Version { major: 2, minor: 0, patch: 0 };
        if (dev.api_version() < min_ver) || (dev.api_version() >= max_ver) {
            return false;
        }

        // Some features may be requested by the user, we need to look at them
        if !dev.supported_features().superset_of(features) {
            return false;
        }

        // Same goes for device extensions
        let unsupported_exts =
            extensions.difference(&DeviceExtensions::supported_by_device(dev));
        if unsupported_exts != DeviceExtensions::none() {
            return false;
        }

        // At least one device queue family should fit our needs
        if dev.queue_families().find(&mut *queue_filter).is_none() {
            return false;
        }

        // Test extra user filtering criteria
        other_criteria(dev)
    }
}