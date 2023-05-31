use std::slice;

use ash::{
    extensions::khr::{self, DynamicRendering},
    vk,
};

use crate::{instance::Instance, physical_device::PhysicalDevice};

pub struct Device {
    pub handle: ash::Device,
    pub queue: vk::Queue,
    pub dynamic_rendering: khr::DynamicRendering,
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
        let dynamic_rendering_ext = ash::extensions::khr::DynamicRendering::name();
        let enabled_extensions = [swapchain_ext.as_ptr(), dynamic_rendering_ext.as_ptr()];

        let handle = unsafe {
            let mut dynamic_rendering =
                vk::PhysicalDeviceDynamicRenderingFeatures::builder().dynamic_rendering(true);
            let mut features = vk::PhysicalDeviceFeatures2::builder()
                .push_next(&mut dynamic_rendering)
                .features(
                    vk::PhysicalDeviceFeatures::builder()
                        .sampler_anisotropy(true)
                        .fill_mode_non_solid(true)
                        .sample_rate_shading(true)
                        .build(),
                );
            instance.handle.create_device(
                physical_device.handle,
                &vk::DeviceCreateInfo::builder()
                    .push_next(&mut features)
                    .queue_create_infos(slice::from_ref(&queue_create_info))
                    .enabled_extension_names(&enabled_extensions),
                None,
            )?
        };

        let queue = unsafe { handle.get_device_queue(queue_family.index, 0) };

        let dynamic_rendering = DynamicRendering::new(&instance.handle, &handle);

        Ok(Device {
            handle,
            queue,
            dynamic_rendering,
        })
    }
}
