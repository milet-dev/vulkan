use ash::vk;

#[derive(Debug, Clone)]
pub struct PhysicalDevice {
    pub handle: vk::PhysicalDevice,
    pub name: String,
    pub queue_families: Vec<(u32, vk::QueueFamilyProperties)>,
}
