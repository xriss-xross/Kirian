use std::time::SystemTime;

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
    RenderPassBeginInfo, SubpassBeginInfo, SubpassContents, SubpassEndInfo,
    AutoCommandBufferBuilder, CommandBufferUsage, CopyImageToBufferInfo
};
use vulkano::command_buffer::allocator::{
    StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo, 
};

use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;

use vulkano::pipeline::layout::PipelineDescriptorSetLayoutCreateInfo;
use vulkano::pipeline::graphics::viewport::{Viewport, ViewportState};
use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
use vulkano::pipeline::graphics::vertex_input::{Vertex, VertexDefinition};
use vulkano::pipeline::graphics::GraphicsPipelineCreateInfo;
use vulkano::pipeline::graphics::color_blend::{ColorBlendAttachmentState, ColorBlendState};
use vulkano::pipeline::graphics::multisample::MultisampleState;
use vulkano::pipeline::graphics::rasterization::RasterizationState;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::{
    PipelineLayout, PipelineShaderStageCreateInfo,
};

use vulkano::render_pass::Subpass;

use image::{ImageBuffer, Rgba};
use vulkano::image::view::ImageView;
use vulkano::image::{Image, ImageCreateInfo, ImageType, ImageUsage};
use vulkano::format::Format;

use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo};

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
    
    let viewport = Viewport {
        offset: [0.0, 0.0],
        extent: [1024.0, 1024.0],
        depth_range: 0.0..=1.0,
    };

    mod vs {
        vulkano_shaders::shader! {
            ty: "vertex",
            src: r"
                #version 460

                layout(location = 0) in vec2 position;

                void main() {
                    gl_Position = vec4(position, 0.0, 1.0);
                }",
        }
    }

    mod fs {
        vulkano_shaders::shader! {
            ty: "fragment",
            src: r"
                #version 460

                layout(location = 0) out vec4 f_color;

                void main() {
                    // bumpin' that
                    f_color = vec4(0.541, 0.808, 0.0, 0.0);
                }",
        }
    }

    let vs = vs::load(device.clone()).expect("Error: failed to create vertex shader");
    let fs = fs::load(device.clone()).expect("Error: failed to create fragment shader");

    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

    let image = Image::new(
        memory_allocator.clone(),
        ImageCreateInfo{
            image_type: ImageType::Dim2d,
            format: Format::R8G8B8A8_UNORM,
            extent: [1024, 1024, 1],
            usage: ImageUsage::COLOR_ATTACHMENT
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
        position: [-0.5, -0.5] };
    let vertex2 = MyVertex {
        position: [ 0.5, -0.5] };
    let vertex3 = MyVertex {
        position: [ 0.0, 0.5] };

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

    let render_pass = vulkano::single_pass_renderpass!(
        device.clone(),
        attachments: {
            color: {
                format: Format::R8G8B8A8_UNORM,
                samples: 1,
                load_op: Clear,
                store_op: Store,
            },
        },
        pass: {
            color: [color],
            depth_stencil: {},
        },
    ).unwrap();
    
    let frame_buffer = Framebuffer::new(
        render_pass.clone(),
        FramebufferCreateInfo {
            attachments: vec![view],
            ..Default::default()
        },
    ).expect("Error: failed to create fame buffer");

    let pipeline = {
        let vs = vs.entry_point("main").unwrap();
        let fs = fs.entry_point("main").unwrap();

        let vertex_input_state = MyVertex::per_vertex()
            .definition(&vs.info().input_interface)
            .unwrap();

        let stages = [
            PipelineShaderStageCreateInfo::new(vs),
            PipelineShaderStageCreateInfo::new(fs),
        ];

        let layout = PipelineLayout::new(
            device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
                .into_pipeline_layout_create_info(device.clone())
                .unwrap(),
        ).unwrap();

        let subpass = Subpass::from(render_pass.clone(), 0).unwrap();

        GraphicsPipeline::new(
            device.clone(),
            None,
            GraphicsPipelineCreateInfo {
                stages: stages.into_iter().collect(),
                vertex_input_state: Some(vertex_input_state),
                input_assembly_state: Some(InputAssemblyState::default()),
                viewport_state: Some(ViewportState {
                    viewports: [viewport].into_iter().collect(),
                    ..Default::default()
                }),
                rasterization_state: Some(RasterizationState::default()),
                multisample_state: Some(MultisampleState::default()),
                color_blend_state: Some(ColorBlendState::with_attachment_states(
                subpass.num_color_attachments(),
                ColorBlendAttachmentState::default(),
                )),
                subpass: Some(subpass.into()),
                ..GraphicsPipelineCreateInfo::layout(layout)
            },
        ).unwrap()
    };


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
        .begin_render_pass(
            RenderPassBeginInfo { 
                clear_values: vec![Some([0.541, 0.808, 0.0, 1.0].into())],
                ..RenderPassBeginInfo::framebuffer(frame_buffer.clone())
            }, SubpassBeginInfo {
                contents: SubpassContents::Inline,
                ..Default::default()
            },
        ).unwrap()

        .bind_pipeline_graphics(pipeline.clone())
        .unwrap()
        .bind_vertex_buffers(0, vertex_buffer.clone())
        .unwrap()
        .draw(
            3, 1, 0, 0,
        ).unwrap()
        
        .end_render_pass(
            SubpassEndInfo::default()
        )
        .unwrap()

        .copy_image_to_buffer(
            CopyImageToBufferInfo::image_buffer(image, image_buffer.clone())
        )
        .unwrap();

    let command_buffer = builder.build().unwrap();

    let future = sync::now(device.clone())
        .then_execute(queue.clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();
    
    let now = SystemTime::now();

    future.wait(None).unwrap();

    let end = now.elapsed();

    let buffer_content = image_buffer.read().unwrap();

    let image_result = ImageBuffer::<Rgba<u8>, _>::from_raw(
        1024, 1024, &buffer_content[..]
    ).expect("Error: could not create image from ImageBuffer");

    image_result.save("image.png").expect("Error: failed to save image to .png file");

    println!("Process: {:?}", end);
}
