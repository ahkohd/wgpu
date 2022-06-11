use std::borrow::Cow;

use env_logger;
use winit::{event_loop::EventLoop, window::WindowBuilder};

mod common;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("var/ch02_first_triangle");
    env_logger::init();

    let inputs = common::Inputs {
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
    };

    pollster::block_on(common::run(event_loop, window, inputs, 3))
}
