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
        let extensions_supported = check_supported_device_extensions(
            instance,
            physical_device,
            &[
                swapchain_ext.to_str().unwrap(),
                dynamic_rendering_ext.to_str().unwrap(),
            ],
        );
        debug_assert!(extensions_supported);

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

fn check_supported_device_extensions(
    instance: &Instance,
    physical_device: &PhysicalDevice,
    required_extensions: &[&str],
) -> bool {
    let extensions = unsafe {
        instance
            .handle
            .enumerate_device_extension_properties(physical_device.handle)
            .unwrap()
    };
    let response: Vec<&str> = required_extensions
        .iter()
        .flat_map(|name| {
            extensions
                .iter()
                .map(|ext| {
                    let bytes: &[u8] = unsafe {
                        std::slice::from_raw_parts(
                            ext.extension_name.as_ptr().cast(),
                            ext.extension_name.len(),
                        )
                    };
                    unsafe { std::str::from_utf8_unchecked(bytes) }.trim_end_matches('\0')
                })
                .filter(|ext_name| name.eq(ext_name))
                .collect::<Vec<&str>>()
        })
        .collect();
    response.eq(required_extensions)
}
