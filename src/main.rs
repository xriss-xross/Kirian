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

    fn main() {
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

        // the meaning of life
        let meaning_iter = 0..42000u32;
        // triangle source and destination buffers
        let _meaning_buffer = Buffer::from_iter(
            memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::STORAGE_BUFFER, // buffer will be used in a compute shader
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter:
                    MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                    ..Default::default()
            },
            meaning_iter,
        )
        .expect("Creating triangle vertices source buffer: failed");    

        mod cs {
            vulkano_shaders::shader!{
                ty: "compute",
                src: r"
                    #version 450
        
                    layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;
        
                    layout(set = 0, binding = 0) buffer Data {
                        uint data[];
                    } buf;
        
                    void main() {
                        uint idx = gl_GlobalInvocationID.x;
                        buf.data[idx] *= 12;
                    }
                ",
            }
        }

    }
