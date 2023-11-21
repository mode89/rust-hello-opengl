extern crate cgmath;
#[macro_use]
extern crate glium;
extern crate winit;

use std::time::{
    Duration,
    Instant,
};

use winit::{
    event::{
        DeviceEvent,
        Event,
        StartCause,
        WindowEvent,
    },
    event_loop::EventLoopBuilder,
};

use glium::{
    backend::glutin::SimpleWindowBuilder,
    index::{
        NoIndices,
        PrimitiveType,
    },
    Program,
    Surface,
    VertexBuffer,
};

use cgmath::{
    Matrix4,
    SquareMatrix,
};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
}
implement_vertex!(Vertex, position);

const FPS: u32 = 60;
const FRAME_TIME: Duration = Duration::from_micros(1_000_000 / FPS as u64);

fn main() {
    let event_loop = EventLoopBuilder::new().build();
    let (_window, display) = SimpleWindowBuilder::new().build(&event_loop);

    let program = Program::from_source(&display,
        &format!(r#"
            #version 140
            in vec3 position;
            uniform mat4 u_model;
            uniform mat4 u_view;
            uniform mat4 u_projection;
            void main() {{
                gl_Position =
                    u_projection *
                    u_view *
                    u_model *
                    vec4(position, 1.0);
            }}
        "#),
        r#"
            #version 140
            uniform vec4 u_color;
            out vec4 color;
            void main() {
                color = u_color;
            }
        "#,
        None
    ).unwrap();

    let line = vec![
        Vertex { position: [-0.5, -0.5, 0.0] },
        Vertex { position: [ 0.5, -0.5, 0.0] },
        Vertex { position: [ 0.0,  0.5, 0.0] },
    ];
    let line_vb = VertexBuffer::new(&display, &line).unwrap();
    let line_ib = NoIndices(PrimitiveType::LineStrip);

    let identity = Matrix4::<f32>::identity();

    event_loop.run(move |ev, _, control_flow| {
        match ev {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => control_flow.set_exit(),
                WindowEvent::Resized(size) => {
                    display.resize(size.into());
                },
                _ => (),
            },
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseMotion { delta } => {
                    println!("Mouse moved by {} {}", delta.0, delta.1);
                },
                _ => (),
            },
            Event::NewEvents(StartCause::Init) => {
                control_flow.set_wait_until(Instant::now() + FRAME_TIME);
            },
            Event::NewEvents(StartCause::ResumeTimeReached { .. }) => {
                control_flow.set_wait_until(Instant::now() + FRAME_TIME);
                let mut target = display.draw();
                target.clear_color(0.1, 0.1, 0.1, 1.0);
                target.draw(
                    &line_vb,
                    &line_ib,
                    &program,
                    &uniform! {
                        u_color: [1.0, 0.0, 0.0, 1.0f32],
                        u_model: mat4(identity),
                        u_view: mat4(identity),
                        u_projection: mat4(identity),
                    },
                    &Default::default()
                ).unwrap();
                target.finish().unwrap();
            },
            _ => (),
        }
    });
}

fn mat4 (m: Matrix4<f32>) -> [[f32; 4]; 4] {
    Into::<[[f32; 4]; 4]>::into(m)
}
