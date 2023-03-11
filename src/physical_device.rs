use ash::vk;

#[derive(Debug, Clone)]
pub struct PhysicalDevice {
    pub handle: vk::PhysicalDevice,
    pub name: String,
}
