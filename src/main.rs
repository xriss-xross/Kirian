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

use vulkano::pipeline::compute::ComputePipelineCreateInfo;
use vulkano::pipeline::layout::PipelineDescriptorSetLayoutCreateInfo;
use vulkano::pipeline::{
    Pipeline, ComputePipeline, PipelineLayout,
    PipelineShaderStageCreateInfo, PipelineBindPoint
};
use vulkano::descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet};
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;

use vulkano::command_buffer::allocator::{
    StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo, 
};
use vulkano::command_buffer::{
    ClearColorImageInfo, AutoCommandBufferBuilder, CommandBufferUsage
};

use vulkano::sync:: {self, GpuFuture};

use vulkano::image::{Image, ImageCreateInfo, ImageType, ImageUsage};
use vulkano::format::{Format, ClearColorValue};
// During development is quite useful since I won't immediately be using variables
#[allow(unused)]
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
        }).expect("Finding graphical queue family: failed") as u32;
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
    ).expect("Creation of device: failed");

    let queue = queues.next().unwrap();
    
    let memory_allocator = Arc::new(
        StandardMemoryAllocator::new_default(device.clone()));

    // the meaning of life
    let meaning_iter = 0..42000u32;
    let meaning_buffer = Buffer::from_iter(
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
    ).expect("Creating meaning to 42,000: failed");

    let image = Image::new(
        memory_allocator.clone(),
        ImageCreateInfo{
            image_type: ImageType::Dim2d,
            format: Format::R8G8B8A8_UNORM,
            extent: [1024, 1024, 1],
            usage: ImageUsage::TRANSFER_DST | ImageUsage::TRANSFER_SRC,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
            ..Default::default()
        },
    )
    .unwrap();

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
        .clear_color_image(ClearColorImageInfo {
            clear_value: ClearColorValue::Float([0.0, 0.0, 1.0, 1.0]),
            ..ClearColorImageInfo::image(image.clone())
        }).unwrap();

    // glsl macro
    mod cs {
        vulkano_shaders::shader!{
            ty: "compute",
            src: r"
                #version 460
    
                layout(local_size_x = 60, local_size_y = 1, local_size_z = 1) in;
    
                layout(set = 0, binding = 0) buffer Data {
                    uint data[];
                } buf;
    
                void main() {
                    uint idx = gl_GlobalInvocationID.x;
                    buf.data[idx] *= 42;
                }
            ",
        }
    }
    let shader = cs::load(device.clone()).expect("failed to create shader module");
    // load() is created when the GLSL is compiled at runtime

    // main entry point
    let cs = shader.entry_point("main").unwrap();
    let stage = PipelineShaderStageCreateInfo::new(cs);
    let layout = PipelineLayout::new(
        device.clone(),
        PipelineDescriptorSetLayoutCreateInfo::from_stages([&stage])
            .into_pipeline_layout_create_info(device.clone())
            .unwrap(),
    ).unwrap();

    // a pipeline containing GLSL code
    let compute_pipeline = ComputePipeline::new(
        device.clone(),
        None,
        ComputePipelineCreateInfo::stage_layout(stage, layout),
    ).expect("Compute pipeline creation: failed");

    let descriptor_set_allocator =
        StandardDescriptorSetAllocator::new(device.clone(), Default::default());
    let pipeline_layout = compute_pipeline.layout();
    let descriptor_set_layouts = pipeline_layout.set_layouts();

    let descriptor_set_layout_index = 0;
    let descriptor_set_layout = descriptor_set_layouts
        .get(descriptor_set_layout_index)
        .unwrap();
    let descriptor_set = PersistentDescriptorSet::new(
        &descriptor_set_allocator,
        descriptor_set_layout.clone(),
        [WriteDescriptorSet::buffer(0, meaning_buffer.clone())],
        [],
    ).unwrap();

    let command_buffer_allocator = StandardCommandBufferAllocator::new(
        device.clone(),
        StandardCommandBufferAllocatorCreateInfo::default(),
    );

    let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
        &command_buffer_allocator,
        queue.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    ).unwrap();

    let work_group_counts = [700, 1, 1];

    command_buffer_builder
        .bind_pipeline_compute(compute_pipeline.clone())
        .unwrap()
        .bind_descriptor_sets(
            PipelineBindPoint::Compute,
            compute_pipeline.layout().clone(),
            descriptor_set_layout_index as u32,
            descriptor_set,
        )
        .unwrap()
        .dispatch(work_group_counts)
        .unwrap();

    let command_buffer = command_buffer_builder.build().unwrap();

    let future = sync::now(device.clone())
        .then_execute(queue.clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();

    future.wait(None).unwrap();


}
