fn main() {
    use vulkano::VulkanLibrary;
    use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo};


    let library = VulkanLibrary::new().expect("Vulkan not installed");
    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            flags:InstanceCreateFlags::ENUMERATE_PORTABILITY,
            ..Default::default()
        },).expect("Instance creation: failed");
    
    let _physical_device = instance
        .enumerate_physical_devices()
        .expect("Enumeration of devices: failed")
        .next()  // chose the first device if any
        .expect("No devices available");
        // it can happen that no devices support Vulkan

}
