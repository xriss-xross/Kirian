fn main() {
    use std::sync::Arc;

    use vulkano::VulkanLibrary;
    use vulkano::instance::{
        Instance, InstanceCreateFlags, InstanceCreateInfo
    };

    use vulkano::device::QueueFlags;
    use vulkano::device::{
        Device, DeviceCreateInfo, QueueCreateInfo
    };
    
    use vulkano::memory::allocator::{
        StandardMemoryAllocator, AllocationCreateInfo, MemoryTypeFilter
    };
    use vulkano::buffer::{
        Buffer, BufferCreateInfo, BufferUsage, //BufferContents
    };


    let lib = VulkanLibrary::new().expect("Vulkan not installed");

    let instance = Instance::new(
        lib,
        InstanceCreateInfo {
            flags:InstanceCreateFlags::ENUMERATE_PORTABILITY,
            ..Default::default()
        },).expect("Instance creation: failed");
    
    let physical_device = instance
        .enumerate_physical_devices()
        .expect("Enumeration of devices: failed")
        .next()  // chose the first device if any
        .expect("No devices available");
        // it can happen that no devices support Vulkan

    let queue_family_index = physical_device
        .queue_family_properties()
        .iter()
        .enumerate()
        .position(|(_queue_family_index, queue_family_properties)| {
            queue_family_properties.queue_flags.contains(QueueFlags::GRAPHICS)
        })
        .expect("Finding graphical queue family: failed") as u32;
        // as u32 because vulkano expects queue_family_index as a u32 not a usize


    let (device, mut queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    )
    .expect("Creation of device: failed");

    let _queue = queues.next().unwrap();
    
    let memory_allocator = Arc::new(
        StandardMemoryAllocator::new_default(device.clone()));

    /*  For example use with from_data
    #[derive(BufferContents)]
    #[repr(C)]
    struct UniverseStruct {
        a: u8,
    }

    let data = UniverseStruct { a: 42 };
    */

    let universe_universe = (0..42).map(|_| 42u8);
    
    // from_iter is for unknown sizes of data
    let _buffer = Buffer::from_iter(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::UNIFORM_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter:
                MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            universe_universe
    )
    .unwrap();
    
}
