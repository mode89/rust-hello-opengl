#[macro_use]
extern crate glium;
extern crate winit;

use winit::event::{
    Event,
    WindowEvent,
};
use winit::event_loop::{
    EventLoopBuilder,
};
use glium::{
    index::{
        NoIndices,
        PrimitiveType,
    },
    Program,
    Surface,
    VertexBuffer,
};
use glium::backend::glutin::SimpleWindowBuilder;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

fn main() {
    let event_loop = EventLoopBuilder::new().build();
    let (_window, display) = SimpleWindowBuilder::new().build(&event_loop);

    let triangle = vec![
        Vertex { position: [-0.5, -0.5] },
        Vertex { position: [ 0.0,  0.5] },
        Vertex { position: [ 0.5, -0.5] },
    ];
    let triangle_vb = VertexBuffer::new(&display, &triangle).unwrap();
    let triangle_ib = NoIndices(PrimitiveType::TrianglesList);

    let program = Program::from_source(&display,
        &format!(r#"
            #version 140
            in vec2 position;
            void main() {{
                gl_Position = vec4(position, 0.0, 1.0);
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

    event_loop.run(move |ev, _, control_flow| {
        match ev {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => control_flow.set_exit(),
                WindowEvent::Resized(size) => {
                    display.resize(size.into());
                },
                _ => (),
            },
            Event::RedrawRequested(_) => {
                let mut target = display.draw();
                target.clear_color(0.3, 0.3, 0.3, 1.0);
                target.draw(
                    &triangle_vb,
                    &triangle_ib,
                    &program,
                    &uniform! {
                        u_color: [1.0, 0.0, 0.0, 1.0f32],
                    },
                    &Default::default()
                ).unwrap();
                target.finish().unwrap();
            },
            _ => (),
        }
    });
}
