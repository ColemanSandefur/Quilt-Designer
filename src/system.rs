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
use std::rc::Rc;
use std::cell::RefCell;

pub struct System {
    pub event_loop: EventLoop<()>,
    pub display: Rc<glium::Display>,
    pub imgui: Context,
    pub platform: WinitPlatform,
    pub glium_renderer: Rc<RefCell<GliumRenderer>>,
    pub font_size: f32,
}

pub fn init(title: &str) -> System {
    //
    // Window
    //

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
    let display = Rc::new(Display::new(builder, context, &event_loop).expect("Failed to initialize display"));
    
    //
    // IMGUI
    //

    let mut imgui = Context::create();
    imgui.set_ini_filename(None);
    
    let mut platform = WinitPlatform::init(&mut imgui);
    {
        let gl_window = display.gl_window();
        let window = gl_window.window();
        platform.attach_window(imgui.io_mut(), window, HiDpiMode::Locked(1.0));
    }
    
    // font setup
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
    
    // connect imgui to glium
    let glium_renderer = GliumRenderer::init(&mut imgui, &*display).expect("Failed to initialize renderer");
    
    // my initializers

    System {
        event_loop,
        display,
        imgui,
        platform,
        glium_renderer: Rc::new(RefCell::new(glium_renderer)),
        font_size,
    }
}
    
impl System {
    pub fn main_loop<
        F: FnMut(&mut bool, &mut glium::Frame, &mut Ui, Rc<RefCell<imgui_glium_renderer::Renderer>>, &dyn glium::backend::Facade) + 'static,
        T: Fn(&glutin::event::WindowEvent) + 'static,
    >(self, mut run_ui: F, window_event_handler: T) {
        let System {
            event_loop,
            display,
            mut imgui,
            mut platform,
            glium_renderer,
            ..
        } = self;
        let mut last_frame = Instant::now();

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
                run_ui(&mut run, &mut target, &mut ui, glium_renderer.clone(), &*display);
                if !run {
                    *control_flow = ControlFlow::Exit;
                }

                let gl_window = display.gl_window();
                platform.prepare_render(&ui, gl_window.window());
                let draw_data = ui.render();
                glium_renderer
                    .borrow_mut()
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
                    window_event_handler(event);
                }
                let gl_window = display.gl_window();
                platform.handle_event(imgui.io_mut(), gl_window.window(), &event);
            }
        })
    }
}
