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

use vulkano::buffer::BufferContents;
use vulkano::buffer::{
    Buffer, BufferCreateInfo, BufferUsage,
};

use vulkano::command_buffer::{
    AutoCommandBufferBuilder, CommandBufferUsage, CopyImageToBufferInfo
};
use vulkano::command_buffer::allocator::{
    StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo, 
};

use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet};

use vulkano::pipeline::compute::ComputePipelineCreateInfo;
use vulkano::pipeline::layout::PipelineDescriptorSetLayoutCreateInfo;
use vulkano::pipeline::graphics::vertex_input::Vertex;
use vulkano::pipeline::{
    ComputePipeline, Pipeline, PipelineBindPoint, PipelineLayout, PipelineShaderStageCreateInfo,
};

use image::{ImageBuffer, Rgba};
use vulkano::image::view::ImageView;
use vulkano::image::{Image, ImageCreateInfo, ImageType, ImageUsage};
use vulkano::format::Format;

use vulkano::sync:: {self, GpuFuture};

#[allow(unused)]
fn main() {
    let lib = VulkanLibrary::new().expect("Vulkan not installed");
    let instance = Instance::new(
        lib,
        InstanceCreateInfo {
            flags:InstanceCreateFlags::ENUMERATE_PORTABILITY,
            ..Default::default()
        },).expect("Erorr: failed to create instance");
    
    let physical_device = instance
        .enumerate_physical_devices()
        .expect("Error: failed to enumerate devices")
        .next()  // chose the first device if any
        .expect("Error: no supported devices found");
        // it can happen that no devices support Vulkan

    // find a queue (threads for GPU) that supports graphical operations
    let queue_family_index = physical_device
        .queue_family_properties()
        .iter()
        .enumerate()
        .position(|(_queue_family_index, queue_family_properties)| {
            queue_family_properties.queue_flags.contains(QueueFlags::GRAPHICS)
        }).expect("Error: failed to find a graphical queue family") as u32;

    let (device, mut queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    ).expect("Error: failed to create device");

    let queue = queues.next().unwrap();
    
    mod cs {
        vulkano_shaders::shader! {
            ty: "compute",
            src: r"
                #version 460

                layout(local_size_x = 8, local_size_y = 8, local_size_z = 1) in;

                layout(set = 0, binding = 0, rgba8) uniform writeonly image2D img;

                void main() {
                    vec2 norm_coordinates = (gl_GlobalInvocationID.xy + vec2(0.5)) / vec2(imageSize(img));

                    // π to 15 digits sufficient for calculations within our solar system
                    float x = norm_coordinates.x * 2 * 3.141592653589793;

                    float y = 0.5 + 0.4 * sin(x * 1.0);

                    float d = abs(norm_coordinates.y - y);

                    float i = smoothstep(0.01, 0.0, d);

                    vec4 to_write = vec4(vec3(i), 1.0);
                    imageStore(img, ivec2(gl_GlobalInvocationID.xy), to_write);
                }
            ",
        }
    }

    let shader = cs::load(device.clone()).expect("Error: failed to create shader module");

    let cs = shader.entry_point("main").unwrap();
    let stage = PipelineShaderStageCreateInfo::new(cs);
    let layout = PipelineLayout::new(
        device.clone(),
        PipelineDescriptorSetLayoutCreateInfo::from_stages([&stage])
            .into_pipeline_layout_create_info(device.clone())
            .unwrap(),
    )
    .unwrap();

    let compute_pipeline = ComputePipeline::new(
        device.clone(),
        None,
        ComputePipelineCreateInfo::stage_layout(stage, layout),
    ).expect("Error: failed to create compute pipeline");

    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

    let image = Image::new(
        memory_allocator.clone(),
        ImageCreateInfo{
            image_type: ImageType::Dim2d,
            format: Format::R8G8B8A8_UNORM,
            extent: [1024, 1024, 1],
            usage: ImageUsage::TRANSFER_DST
            | ImageUsage::TRANSFER_SRC
            | ImageUsage::STORAGE,  // different from previous
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
            ..Default::default()
        },
    ).expect("Error: failed to create image");

    let view = ImageView::new_default(image.clone()).unwrap();

    let descriptor_set_allocator =
        StandardDescriptorSetAllocator::new(device.clone(), Default::default());

    let layout = compute_pipeline.layout().set_layouts().get(0).unwrap();

    let set = PersistentDescriptorSet::new(
        &descriptor_set_allocator,
        layout.clone(),
        [WriteDescriptorSet::image_view(0, view.clone())],
        [],
    ).unwrap();

    let image_buffer = Buffer::from_iter(
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
        // *4 because not actually no. of bits but no. of elements
        (0..1024 * 1024 * 4).map(|_| 0u8),
    ).expect("Error: failed to create image buffer");

    #[derive(BufferContents, Vertex)]
    #[repr(C)]
    struct MyVertex {
        #[format(R32G32_SFLOAT)]
        position: [f32; 2],
    }

    let vertex1 = MyVertex {
        position: [-0.5, 0.5] };
    let vertex2 = MyVertex {
        position: [ 0.0, -0.5] };
    let vertex3 = MyVertex {
        position: [ 0.5, 0.5] };

    let vertex_buffer = Buffer::from_iter(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::VERTEX_BUFFER,
            ..Default::default()
        }, 
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
            | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        }, vec![vertex1, vertex2, vertex3]
    ).expect("Error: failed to create vertex buffer");

    let command_buffer_allocator = StandardCommandBufferAllocator::new(
        device.clone(),
        StandardCommandBufferAllocatorCreateInfo::default(),
    );

    let mut builder = AutoCommandBufferBuilder::primary(
        &command_buffer_allocator,
        queue.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    ).unwrap();

    builder
        .bind_pipeline_compute(compute_pipeline.clone())
        .unwrap()
        .bind_descriptor_sets(
            PipelineBindPoint::Compute,
            compute_pipeline.layout().clone(),
            0,
            set,
        )
        .unwrap()
        .dispatch([1024 / 8, 1024 / 8, 1])
        .unwrap()
        .copy_image_to_buffer(CopyImageToBufferInfo::image_buffer(
            image.clone(),
            image_buffer.clone(),
        ))    
        .unwrap();

    let command_buffer = builder.build().unwrap();

    let future = sync::now(device.clone())
        .then_execute(queue.clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();
    future.wait(None).unwrap();

    let buffer_content = image_buffer.read().unwrap();

    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(
        1024, 1024, &buffer_content[..]
    ).unwrap();

    image.save("image.png").expect("Error: failed to save image to .png file");
}
