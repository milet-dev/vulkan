use std::slice;

use ash::{extensions::khr, vk};

use crate::{instance::Instance, physical_device::PhysicalDevice};

pub struct Device {
    pub handle: ash::Device,
    pub queue: vk::Queue,
}

impl Device {
    pub fn new(instance: &Instance, physical_device: &PhysicalDevice) -> anyhow::Result<Device> {
        let queue_family = physical_device
            .queue_families
            .iter()
            .copied()
            .find(|queue_family| {
                queue_family
                    .properties
                    .queue_flags
                    .contains(vk::QueueFlags::GRAPHICS)
            })
            .unwrap();

        let queue_create_info = vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(queue_family.index)
            .queue_priorities(&[1.0]);

        let swapchain_ext = khr::Swapchain::name();

        let handle = unsafe {
            instance.handle.create_device(
                physical_device.handle,
                &vk::DeviceCreateInfo::builder()
                    .queue_create_infos(slice::from_ref(&queue_create_info))
                    .enabled_extension_names(slice::from_ref(&swapchain_ext.as_ptr())),
                None,
            )?
        };

        let queue = unsafe { handle.get_device_queue(queue_family.index, 0) };

        Ok(Device { handle, queue })
    }
}
