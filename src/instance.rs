use std::ffi::CString;

use ash::{extensions::khr, vk};

use crate::physical_device::{PhysicalDevice, QueueFamily};

const APP_NAME: &str = "vulkan";
const ENGINE_NAME: &str = "vulkan";
const VALIDATION_LAYER: &str = "VK_LAYER_KHRONOS_validation";

pub struct Instance {
    pub entry: ash::Entry,
    pub handle: ash::Instance,
}

impl Instance {
    pub fn new() -> anyhow::Result<Self> {
        let entry = unsafe { ash::Entry::load()? };

        let app_name = CString::new(APP_NAME)?;
        let engine_name = CString::new(ENGINE_NAME)?;
        let validation_layer = CString::new(VALIDATION_LAYER)?;

        let mut enabled_layers = Vec::new();
        let mut enabled_extensions = vec![khr::Surface::name().as_ptr()];

        if cfg!(debug_assertions) {
            enabled_layers.push(validation_layer.as_ptr());
        }
        if cfg!(windows) {
            enabled_extensions.push(khr::Win32Surface::name().as_ptr());
        }

        let handle = unsafe {
            entry.create_instance(
                &vk::InstanceCreateInfo::builder()
                    .application_info(
                        &vk::ApplicationInfo::builder()
                            .application_name(&app_name)
                            .application_version(ash::vk::make_api_version(0, 1, 0, 0))
                            .engine_name(&engine_name)
                            .engine_version(ash::vk::make_api_version(0, 1, 0, 0))
                            .api_version(vk::API_VERSION_1_2),
                    )
                    .enabled_layer_names(enabled_layers.as_slice())
                    .enabled_extension_names(enabled_extensions.as_slice()),
                None,
            )?
        };

        Ok(Self { entry, handle })
    }

    pub fn physical_devices(&self) -> anyhow::Result<Vec<PhysicalDevice>> {
        let physical_devices = unsafe { self.handle.enumerate_physical_devices()? };
        Ok(physical_devices
            .into_iter()
            .map(|handle| {
                let properties = unsafe { self.handle.get_physical_device_properties(handle) };
                let name = {
                    let device_name = properties.device_name;
                    let bytes: &[u8] = unsafe {
                        std::slice::from_raw_parts(device_name.as_ptr().cast(), device_name.len())
                    };
                    String::from_utf8_lossy(bytes)
                        .trim_end_matches('\0')
                        .to_owned()
                };
                let queue_families: Vec<QueueFamily> = {
                    let queue_families = unsafe {
                        self.handle
                            .get_physical_device_queue_family_properties(handle)
                    };
                    queue_families
                        .into_iter()
                        .enumerate()
                        .map(|(index, queue_family_properties)| QueueFamily {
                            index: index as u32,
                            properties: queue_family_properties,
                        })
                        .collect()
                };
                PhysicalDevice {
                    handle,
                    name,
                    properties,
                    queue_families,
                }
            })
            .collect())
    }
}
