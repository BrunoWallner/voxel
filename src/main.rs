use glium::glutin;
use glium::Surface;
use glium::implement_vertex;
use glium::uniform;

mod shape;
use shape::Triangle;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

 
pub fn main() {
    let window_size: [u32; 2] = [1200, 800];
    
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    //shaders
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        uniform mat4 matrix;

        void main() {
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 1.0, 1.0, 1.0);
        }
    "#;
    let ship_uniform = uniform! {
        matrix: [
            [1.0, 0.0, 0.0, 0.0], //rotation
            [0.0, 1.0, 0.0, 0.0], //scale
            [0.0, 0.0, 0.5, 0.0], //IDK
            [0.0, 0.0, 0.0, 1.0f32], //position
        ]
    };

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let ship_speed: f32 = 7.5;
    let ship_size: [u32; 2] = [1, 1];
    let mut ship_acceleration: [f32; 2] = [0.0, 0.0]; 
    let mut ship_position: [f32; 2] = [575.0, 725.0];

    event_loop.run(move |ev, _, control_flow| {
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        //accelerates ship
        let ship_bounce_back: f32 = 5.0;
        if ship_acceleration[0] < 0.0 {
            if ship_position[0] > 0.0 {
                ship_position[0] += ship_acceleration[0];
            }
            else {
                ship_acceleration[0] = ship_bounce_back;
            }
        }
        else {
            if ship_position[0] < (window_size[0] - ship_size[0]) as f32 {
                ship_position[0] += ship_acceleration[0];
            }
            else {
                ship_acceleration[0] = -ship_bounce_back
            }
        }

        if ship_acceleration[1] < 0.0 {
            if ship_position[1] > 0.0 {
                ship_position[1] += ship_acceleration[1];
            }
            else {
                ship_acceleration[1] = ship_bounce_back;
            }
        }
        else {
            if ship_position[1] < (window_size[1] - ship_size[1]) as f32 {
                ship_position[1] += ship_acceleration[1];
            }
            else {
                ship_acceleration[1] = -ship_bounce_back;
            }
        }

        //friction Physics simulation
        if ship_acceleration[0] != 0.0 {ship_acceleration[0] /= 1.025}
        if ship_acceleration[1] != 0.0 {ship_acceleration[1] /= 1.025}

        //let ship = Triangle::new([ship_position[0], ship_position[1] + (ship_size[0] / 2) as f32], [ship_position[0] - (ship_size[0] / 2) as f32, ship_position[1] - (ship_size[1] / 2) as f32], [ship_position[0] + (ship_size[0] / 2) as f32, ship_position[1] + (ship_size[1] / 2) as f32]);
        let ship = Triangle::new([0.0, (ship_size[1] as f32 / 2.0) as f32], [-(ship_size[0] as f32 / 2.0), -(ship_size[1] as f32 / 2.0)], [ship_size[0] as f32 / 2.0, -(ship_size[1] as f32 / 2.0)]);
        let ship_vertex_buffer = glium::VertexBuffer::new(&display, &ship).unwrap();

        target.draw(&ship_vertex_buffer, &indices, &program, &ship_uniform,
        &Default::default()).unwrap();

        target.finish().unwrap();
    });
}

use std::{thread, time};

pub fn sleep(millis: u64) {
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}
