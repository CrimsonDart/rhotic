use vulkano::{VulkanLibrary, instance::{Instance, InstanceCreateInfo}, device::{QueueFamilyProperties, Device, DeviceCreateInfo, QueueCreateInfo}, memory::allocator::{StandardMemoryAllocator, AllocationCreateInfo}, buffer::{Buffer, BufferCreateInfo, BufferUsage}, command_buffer::{allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo}, AutoCommandBufferBuilder, CopyBufferInfo, self}, shader};
use vulkano::sync::{self, GpuFuture};
use vulkano::device::QueueFlags;
use std::iter::Iterator;

use crate::display::shaders;


pub fn get_graphics() {

    let library = VulkanLibrary::new().expect("No local Vulkan library :(");
    let instance = Instance::new(library, InstanceCreateInfo::default())
        .expect("failed to create instance :(");

    let physical_device = instance
        .enumerate_physical_devices()
        .expect("coulnt enumerate devices.")
        .next()
        .expect("no devices available.");


    let queue_family_index = physical_device
        .queue_family_properties()
        .iter()
        .enumerate()
        .position(|(_queue_family_index, queue_family_properties)| {
            queue_family_properties.queue_flags.contains(QueueFlags::GRAPHICS)
        })
        .expect("couldn't find a graphical queue family") as u32;

    let (device, mut queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            queue_create_infos: vec!(
                QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }
            ),
            ..Default::default()
        }
    ).expect("device creation failed :(");

    let queue = queues.next().unwrap();

    let memory_allocator = StandardMemoryAllocator::new_default(device.clone());
    let source_content: Vec<_> = (0..64).collect();
    let source = Buffer::from_iter(
        &memory_allocator,
        BufferCreateInfo {
            usage: BufferUsage::TRANSFER_SRC,
            ..Default::default()
        },
        AllocationCreateInfo {
            usage: vulkano::memory::allocator::MemoryUsage::Upload,
            ..Default::default()
        },
        source_content
    )
    .expect("failed buffer creation.");

    let destination_content: Vec<_> = Vec::from([0; 64]);
    let destination = Buffer::from_iter(
        &memory_allocator,
        BufferCreateInfo {
            usage: BufferUsage::TRANSFER_DST,
            ..Default::default()
        },
        AllocationCreateInfo {
            usage: vulkano::memory::allocator::MemoryUsage::Download,
            ..Default::default()
        },
        destination_content
    ).expect("destination bufffer failed");


    let command_bufer_allocator = StandardCommandBufferAllocator::new(
        device.clone(),
        StandardCommandBufferAllocatorCreateInfo::default()
    );

    let builder = AutoCommandBufferBuilder::primary(&command_bufer_allocator, queue_family_index, vulkano::command_buffer::CommandBufferUsage::OneTimeSubmit);
    let mut builder = builder.unwrap();

    builder
        .copy_buffer(CopyBufferInfo::buffers(source.clone(), destination.clone())).unwrap();

    let command_buffer = builder.build().unwrap();

    let future = sync::now(device.clone())
        .then_execute(queue.clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();

    future.wait(None).unwrap();


    let shader = shaders::ex::cs::load(device.clone()).unwrap();



    let src_content = source.read().unwrap();
    let destination_content = destination.read().unwrap();
    assert_eq!(&*src_content, &*destination_content);

    println!("Everything succeeded!");










}
