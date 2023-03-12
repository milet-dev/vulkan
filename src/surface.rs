use ash::{extensions::khr, vk};

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

use crate::instance::Instance;

pub struct Surface {
    pub handle: vk::SurfaceKHR,
    pub loader: khr::Surface,
}

impl Surface {
    pub fn new(window: &impl HasRawWindowHandle, instance: &Instance) -> anyhow::Result<Self> {
        let RawWindowHandle::Win32(window_handle) = window.raw_window_handle() else {
            unimplemented!();
        };

        let handle = unsafe {
            khr::Win32Surface::new(&instance.entry, &instance.handle).create_win32_surface(
                &vk::Win32SurfaceCreateInfoKHR::builder()
                    .hinstance(window_handle.hinstance)
                    .hwnd(window_handle.hwnd),
                None,
            )?
        };

        let loader = khr::Surface::new(&instance.entry, &instance.handle);

        Ok(Self { handle, loader })
    }
}
