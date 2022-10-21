
use std::{time::{Instant, Duration}};

use glium::{
    glutin::{
        event_loop::{
            EventLoop, 
            ControlFlow
        }, 
        window::WindowBuilder, 
        ContextBuilder, 
        event::{
            WindowEvent, 
            Event, 
            StartCause
        }
    }, 
    Display, 
    Program, 
    Surface, VertexBuffer,
};

use crate::vertex::*;
use crate::components::*;

static mut VERTEX_BUFFER: Vec<Vertex> = Vec::new();

pub struct App {
    startup_sys: Vec<fn()>,
    update_sys: Vec<fn()>,
    clear_color: (f32, f32, f32),
}

impl App {
    pub fn new () -> Self {
        Self {
            startup_sys: Vec::new(),
            update_sys: Vec::new(),
            clear_color: (0.0, 0.0, 0.0),
        }
    }

    pub fn add_startup_system (&mut self, f: fn()) -> &mut Self {
        self.startup_sys.push(f);
        self
    }

    pub fn add_system (&mut self, f: fn()) -> &mut Self {
        self.update_sys.push(f);
        self
    }

    pub fn with_clear_color (&mut self, color: (f32, f32, f32)) -> &mut Self {
        self.clear_color = color;
        self
    }

    pub fn run (&mut self) {
        let el = EventLoop::new();
        let wb = WindowBuilder::new();
        let cb = ContextBuilder::new();
        let display = Display::new(wb, cb, &el).unwrap();

        let vertex_shader_src = include_str!("shader/vs.glsl");
        let fragment_shader_src = include_str!("shader/fs.glsl");

        let program = Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

        let vert1 = Vertex::new([0.5, 0.5]);
        let vert2 = Vertex::new([0.5, 0.0]);
        let vert3 = Vertex::new([0.0, 0.0]);
        let shape = vec![vert1, vert2, vert3];

        let translation = Translation::new(0., 0.);

        let vertices: VertexBuffer<Vertex> = VertexBuffer::new(&display, &shape).unwrap();

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let clear_color = self.clear_color;
        let update_sys = self.update_sys.clone();

        let mut camera = Camera::new(0.5, 0.5);

        for sys in self.startup_sys.iter() {
            sys();
        }

        el.run(move |event, _, control_flow| {
            let next_frame_time = Instant::now() +
                Duration::from_nanos(16_666_667);
            *control_flow = ControlFlow::WaitUntil(next_frame_time);

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit
                    },
                    WindowEvent::KeyboardInput { input, .. } => {
                        unsafe { crate::input::INPUT_CODE = input.virtual_keycode }
                    }
                    _ => {
                        unsafe { crate::input::INPUT_CODE = None }
                        return
                    }
                },
                Event::NewEvents(cause) => match cause {
                    StartCause::ResumeTimeReached { .. } => (),
                    StartCause::Init { .. } => (),
                    _ => return,
                }
                _ => return,
            }

            let mut target = display.draw();
            target.clear_color(clear_color.0, clear_color.1, clear_color.2, 1.0);

            for sys in update_sys.iter() {
                sys()
            }

            target.draw(&vertices, &indices, &program, 
            &uniform! { perspective: camera.perspective, translation: translation.translation }, 
            &Default::default()).unwrap();
            target.finish().unwrap();

            camera.move_horizontal(-0.01);
            camera.move_vertical(-0.01);

            unsafe { crate::input::LAST_CODE = crate::input::INPUT_CODE }
        });
    }
}