fn main() {
    use std::sync::Arc;

    use vulkano::VulkanLibrary;
    use vulkano::instance::{
        Instance, InstanceCreateFlags, InstanceCreateInfo,
    };

    use vulkano::device::QueueFlags;
    use vulkano::device::{
        Device, DeviceCreateInfo, QueueCreateInfo,
    };
    
    use vulkano::memory::allocator::{
        StandardMemoryAllocator, AllocationCreateInfo, MemoryTypeFilter,
    };
    use vulkano::buffer::{
        Buffer, BufferCreateInfo, BufferUsage,
    };

    use vulkano::command_buffer::allocator::StandardCommandBufferAllocator;
    use vulkano::command_buffer:: {
        AutoCommandBufferBuilder, CommandBufferUsage, CopyBufferInfo,
    };

    use vulkano::sync:: {self, GpuFuture};

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

    let queue = queues.next().unwrap();
    
    let memory_allocator = Arc::new(
        StandardMemoryAllocator::new_default(device.clone()));

    let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
        device.clone(),
        Default::default(),
    ));

    let mut builder = AutoCommandBufferBuilder::primary(
        command_buffer_allocator.clone(),
        queue_family_index,
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();


    // a triangle
    let src_content: Vec<f32> = vec![
        -0.5, -0.5, 0.0,
         0.5, -0.5, 0.0,
         0.0,  0.5, 0.0
    ];

    let dst_content: Vec<f32> = vec![
         0.0,  0.0, 0.0,
         0.0,  0.0, 0.0, 
         0.0,  0.0, 0.0,
    ];

    // triangle source and destination buffers
    let triangle_src = Buffer::from_iter(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::TRANSFER_SRC,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter:
                MemoryTypeFilter::PREFER_HOST | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
        },
        src_content
    )
    .expect("Creating triangle vertices source buffer: failed");
    
    let triangle_dst = Buffer::from_iter(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::TRANSFER_DST,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter:
                MemoryTypeFilter::PREFER_HOST | MemoryTypeFilter::HOST_RANDOM_ACCESS,
                ..Default::default()
        },
        dst_content
    )
    .expect("Creating triangle verticies destination buffer: failed");


    builder
    .copy_buffer(CopyBufferInfo::buffers(triangle_src.clone(), triangle_dst.clone()))
    .unwrap();

    let command_buffer = builder.build().unwrap();

    let future = sync::now(device.clone())
        .then_execute(queue.clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();

    future.wait(None).unwrap();

    let read_src_content = triangle_src.read().unwrap();
    let read_dst_content = triangle_dst.read().unwrap();
    
    // check if the content has actually been copied
    assert_eq!(&*read_src_content, &*read_dst_content);
    println!("Copying: succeeded")

}
