use ash::{extensions::khr, vk};

use crate::{
    device::Device, instance::Instance, physical_device::PhysicalDevice, surface::Surface,
};

pub struct Swapchain {
    pub handle: vk::SwapchainKHR,
    pub loader: khr::Swapchain,
}

impl Swapchain {
    pub fn new(
        instance: &Instance,
        surface: &Surface,
        physical_device: &PhysicalDevice,
        device: &Device,
    ) -> anyhow::Result<Self> {
        let surface_capabilities = unsafe {
            surface
                .loader
                .get_physical_device_surface_capabilities(physical_device.handle, surface.handle)?
        };

        let present_mode = {
            let present_modes = unsafe {
                surface.loader.get_physical_device_surface_present_modes(
                    physical_device.handle,
                    surface.handle,
                )?
            };
            present_modes
                .into_iter()
                .find(|present_mode| present_mode.eq(&vk::PresentModeKHR::MAILBOX))
                .unwrap_or(vk::PresentModeKHR::FIFO)
        };

        let surface_format = {
            let surface_formats = unsafe {
                surface
                    .loader
                    .get_physical_device_surface_formats(physical_device.handle, surface.handle)?
            };
            surface_formats
                .into_iter()
                .find(|surface_format| {
                    surface_format.format.eq(&vk::Format::B8G8R8A8_SRGB)
                        && surface_format
                            .color_space
                            .eq(&vk::ColorSpaceKHR::SRGB_NONLINEAR)
                })
                .unwrap()
        };

        let loader = khr::Swapchain::new(&instance.handle, &device.handle);
        let handle = unsafe {
            loader.create_swapchain(
                &vk::SwapchainCreateInfoKHR::builder()
                    .surface(surface.handle)
                    .min_image_count(surface_capabilities.min_image_count)
                    .image_format(surface_format.format)
                    .image_color_space(surface_format.color_space)
                    .image_extent(surface_capabilities.current_extent)
                    .image_array_layers(surface_capabilities.max_image_array_layers)
                    .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
                    .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
                    .pre_transform(surface_capabilities.current_transform)
                    .queue_family_indices(&[0])
                    .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
                    .present_mode(present_mode)
                    .clipped(false),
                None,
            )?
        };

        Ok(Self { handle, loader })
    }
}
