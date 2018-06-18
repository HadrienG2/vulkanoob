//! Conveniences for creating and manipulating Vulkan devices

use ::Result;

use std::{
    cmp::Ordering,
    sync::Arc,
};

use vulkano::{
    device::{
        Device,
        Queue,
    },
    instance::{
        DeviceExtensions,
        Features,
        PhysicalDevice,
        QueueFamily,
    },
};


/// A convenience wrapper for quickly setting up Vulkan devices
pub struct EasyPhysicalDevice<'instance> {
    /// Wrapped PhysicalDevice
    device: PhysicalDevice<'instance>,
}

impl<'instance> EasyPhysicalDevice<'instance> {
    /// Build an EasyPhysicalDevice by wrapping a vulkano PhysicalDevice
    pub(crate) fn new(device: PhysicalDevice<'instance>) -> Self {
        EasyPhysicalDevice {
            device,
        }
    }

    /// Access the inner Vulkan PhysicalDevice
    pub fn physical_device(&self) -> &PhysicalDevice {
        &self.device
    }

    /// Setup a logical device in a single-queue configuration
    ///
    /// The use of multiple command queues is key to making the most of the
    /// Vulkan API. But during prototyping, it is often convenient to stick with
    /// a simpler single-queue setup. This method provides you with such a setup
    /// with minimal fuss.
    ///
    /// Note that if you used EasyInstance::select_physical_device() to pick
    /// your physical device, you may want to integrate your queue
    /// filter/preference into your device filter/preference.
    ///
    pub fn setup_single_queue_device(
        &self,
        features: &Features,
        extensions: &DeviceExtensions,
        filter: impl Fn(&QueueFamily) -> bool,
        preference: impl Fn(&QueueFamily, &QueueFamily) -> Ordering
    ) -> Result<Option<(Arc<Device>, Arc<Queue>)>> {
        // Select the appropriate queue family (if any)
        if let Some(queue_family) = self.device.queue_families()
                                               .filter(filter)
                                               .max_by(preference)
        {
            // Build a single-queue device
            let (device, mut queues_iter) = Device::new(
                self.device,
                features,
                extensions,
                [(queue_family, 1.0)].iter().cloned()
            )?;

            // Extract the only queue from the iterator (should always succeed,
            // if not it is a bug in vulkano or the Vulkan implementation)
            let queue = queues_iter.next().unwrap();
            assert!(queues_iter.next().is_none());

            // And now we can return the device and the queue
            Ok(Some((device, queue)))
        } else {
            // No suitable queue family was found :-/
            Ok(None)
        }
    }
}