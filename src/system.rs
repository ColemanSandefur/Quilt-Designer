use crate::render::renderer::Renderer;

use glium::glutin;
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::{Display};
use imgui::{Context, FontConfig, FontGlyphRanges, FontSource, Ui};
use imgui_glium_renderer::Renderer as GliumRenderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::path::Path;
use std::time::Instant;

pub struct System {
    pub event_loop: EventLoop<()>,
    pub display: glium::Display,
    pub imgui: Context,
    pub platform: WinitPlatform,
    pub glium_renderer: GliumRenderer,
    pub renderer: Renderer,
    pub font_size: f32,
}

pub fn init(title: &str) -> System {
    let title = match Path::new(&title).file_name() {
        Some(file_name) => file_name.to_str().unwrap(),
        None => title,
    };
    let event_loop = EventLoop::new();
    let context = glutin::ContextBuilder::new()
        .with_depth_buffer(24)
        .with_multisampling(16)
        .with_vsync(true);
    let builder = WindowBuilder::new()
    .with_title(title.to_owned())
    .with_inner_size(glutin::dpi::LogicalSize::new(1024f64, 768f64));
    let display =
    Display::new(builder, context, &event_loop).expect("Failed to initialize display");
    
    crate::render::material::material_manager::initialize_material_manager(&display);
    
    let mut imgui = Context::create();
    imgui.set_ini_filename(None);
    
    let mut platform = WinitPlatform::init(&mut imgui);
    {
        let gl_window = display.gl_window();
        let window = gl_window.window();
        platform.attach_window(imgui.io_mut(), window, HiDpiMode::Locked(1.0));
    }
    
    let hidpi_factor = platform.hidpi_factor();
    let font_size = (13.0 * hidpi_factor) as f32;
    imgui.fonts().add_font(&[
        FontSource::DefaultFontData {
            config: Some(FontConfig {
                size_pixels: font_size,
                ..FontConfig::default()
            }),
        },
        FontSource::TtfData {
            data: include_bytes!("Roboto-Regular.ttf"),
            size_pixels: font_size,
            config: Some(FontConfig {
                rasterizer_multiply: 1.75,
                glyph_ranges: FontGlyphRanges::japanese(),
                ..FontConfig::default()
            }),
        },
        ]);
        
        imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;
        
        let glium_renderer = GliumRenderer::init(&mut imgui, &display).expect("Failed to initialize renderer");
        
        let renderer = Renderer::new(&display, &glium_renderer);

    System {
        event_loop,
        display,
        imgui,
        platform,
        glium_renderer,
        renderer,
        font_size,
    }
}

impl System {
    pub fn main_loop<
        F: FnMut(&mut bool, &mut glium::Frame, &mut Renderer, &mut Ui, &mut imgui_glium_renderer::Renderer, &dyn glium::backend::Facade) + 'static>(self, mut run_ui: F) {
        let System {
            event_loop,
            display,
            mut imgui,
            mut platform,
            mut glium_renderer,
            mut renderer,
            ..
        } = self;
        let mut last_frame = Instant::now();

        crate::quilt::square::block_manager::load_textures(&display, &mut glium_renderer);

        event_loop.run(move |event, _, control_flow| match event {
            Event::NewEvents(_) => {
                let now = Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
            }
            Event::MainEventsCleared => {
                let gl_window = display.gl_window();
                platform
                    .prepare_frame(imgui.io_mut(), gl_window.window())
                    .expect("Failed to prepare frame");
                gl_window.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                let mut ui = imgui.frame();
                let mut target = display.draw();

                let mut run = true;
                run_ui(&mut run, &mut target, &mut renderer, &mut ui, &mut glium_renderer, &display);
                if !run {
                    *control_flow = ControlFlow::Exit;
                }

                

                let gl_window = display.gl_window();
                // target.clear_color_srgb(1.0, 1.0, 1.0, 1.0);
                platform.prepare_render(&ui, gl_window.window());
                let draw_data = ui.render();
                glium_renderer
                    .render(&mut target, draw_data)
                    .expect("Rendering failed");
                target.finish().expect("Failed to swap buffers");
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            event => {
                if let Event::WindowEvent {event, ..} = &event {
                    if let WindowEvent::KeyboardInput {input, ..} = event {
                        if let Some(keycode) = input.virtual_keycode {
                            renderer.keyboard_tracker.set_pressed(keycode, input.state == glutin::event::ElementState::Pressed);
                        }
                    }

                    if let WindowEvent::CursorMoved{position, ..} = &event {
                        renderer.cursor_moved(position);
                    }

                    if let WindowEvent::Focused(is_focused) = event {
                        if !is_focused {
                            renderer.keyboard_tracker.release_all();
                        }
                    }
                }
                let gl_window = display.gl_window();
                platform.handle_event(imgui.io_mut(), gl_window.window(), &event);
            }
        })
    }
}
