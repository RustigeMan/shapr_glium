#[macro_use]
extern crate glium;
extern crate shapr;

pub mod shaders;

use std::time::{Duration, Instant};

use glium::glutin;
use glium::index::{NoIndices, PrimitiveType};
use glium::texture::{MipmapsOption, RawImage1d, Texture1d, UncompressedFloatFormat};
//use glium::uniforms::EmptyUniforms;
use glium::{Display, Program, Surface, VertexBuffer};

use glutin::dpi::LogicalSize;
use glutin::event::{Event, KeyboardInput, StartCause, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

use shapr::Shp;

//const FPS: u64 = 60;
const NS_PER_S: f32 = 1_000_000_000.0;
//const NS_PER_FRAME: u64 = NS_PER_S as u64 / FPS;

#[derive(Copy, Clone, Debug)]
pub struct Vtx {
    position: [f32; 2],
}

impl Vtx {
    fn new(x: f32, y: f32) -> Self {
        Vtx { position: [x, y] }
    }
}
implement_vertex!(Vtx, position);

pub struct AppConfig {
    pub fps: u64,
    pub resolution: [u32; 2],
    pub title: String,
}

impl AppConfig {
    pub fn new_with_title(title: &str) -> Self {
        AppConfig::default().title(title)
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            fps: 60,
            resolution: [800, 800],
            title: "Shapr-Glium App".to_string(),
        }
    }
}

pub trait ShaprGliumApp {
    fn config(&self) -> AppConfig {
        AppConfig::default()
    }

    #[allow(unused)] // Allow default method to not use window argument without warnings
    fn process_event(
        &mut self,
        event: Event<()>,
        window: &EventLoopWindowTarget<()>,
        control_flow: &mut ControlFlow,
    ) {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    println!("Close requested window event received, exiting...");
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                WindowEvent::KeyboardInput { input, .. } => match input {
                    KeyboardInput {
                        virtual_keycode, ..
                    } => match virtual_keycode {
                        Some(keycode) => match keycode {
                            VirtualKeyCode::Escape => {
                                println!("Escape key pressed, exiting...");
                                *control_flow = ControlFlow::Exit;
                                return;
                            }
                            _ => return,
                        },
                        None => return,
                    },
                },
                _ => return,
            },
            _ => return,
        }
    }

    fn draw_frame(&mut self, dt: Duration) -> Shp;
}

pub fn initialize_glium(
    config: &AppConfig,
) -> (EventLoop<()>, Display, Program, VertexBuffer<Vtx>) {
    let event_loop = EventLoop::new();
    let [width, height] = config.resolution;

    let win_builder = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(width, height))
        .with_title(&config.title);

    let ctx_builder = ContextBuilder::new();

    let display =
        Display::new(win_builder, ctx_builder, &event_loop).expect("Could not create display...");

    let shape = vec![
        Vtx::new(-1.0, -1.0),
        Vtx::new(01.0, -1.0),
        Vtx::new(-1.0, 01.0),
        Vtx::new(01.0, 01.0),
        Vtx::new(01.0, -1.0),
        Vtx::new(-1.0, 01.0),
    ];

    let vertex_buffer =
        VertexBuffer::new(&display, &shape).expect("Could not create vertex buffer");
    let fshader = std::fs::read_to_string("./src/shaders/fragment.glsl").unwrap();
    let program = Program::from_source(&display, shaders::VERTEX, &fshader, None)
        .expect("Could not create shader program");

    (event_loop, display, program, vertex_buffer)
}

pub fn run_app<T>(mut app: T)
where
    T: 'static + ShaprGliumApp,
{
    let config = app.config();
    let (event_loop, display, program, vertex_buffer) = initialize_glium(&config);
    let ns_per_frame = NS_PER_S as u64 / config.fps;

    let mut prev_frame_time = Instant::now();
    event_loop.run(move |event, window, control_flow| match event {
        Event::NewEvents(cause) => match cause {
            StartCause::Init | StartCause::ResumeTimeReached { .. } => {
                let now = Instant::now();
                let next_frame_time = now + Duration::from_nanos(ns_per_frame);
                *control_flow = ControlFlow::WaitUntil(next_frame_time);

                let floats = app.draw_frame(now - prev_frame_time).to_float_vector();
                prev_frame_time = now;
                //println!("{:?}", floats);
                let raw_img = RawImage1d::from_raw_rgb(floats);
                let shapes = Texture1d::with_format(
                    &display,
                    raw_img,
                    UncompressedFloatFormat::F32F32F32,
                    MipmapsOption::NoMipmap,
                )
                .expect("Could not create shapes texture");

                let mut target = display.draw();
                //target.clear_color(0.0, 0.0, 0.0, 1.0);
                target
                    .draw(
                        &vertex_buffer,
                        &NoIndices(PrimitiveType::TrianglesList),
                        &program,
                        &uniform! {shapes: &shapes},
                        &Default::default(),
                    )
                    .expect("Draw failed");
                target.finish().expect("Could not finish target");
            }
            _ => app.process_event(event, window, control_flow),
        },
        _ => app.process_event(event, window, control_flow),
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
