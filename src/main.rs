#[macro_use]
extern crate glium;
mod render;

use core::ffi::c_void;
use gio::prelude::*;
use glium::backend::Context;
use glium::SwapBuffersError;
use gtk::prelude::*;
use render::renderer::Renderer;
use std::rc::Rc;
use std::sync::Mutex;

struct GLAreaBackend {
    glarea: gtk::GLArea,
}

unsafe impl glium::backend::Backend for GLAreaBackend {
    fn swap_buffers(&self) -> Result<(), SwapBuffersError> {
        // GTK swaps the buffers after each "render" signal itself
        Ok(())
    }
    unsafe fn get_proc_address(&self, symbol: &str) -> *const c_void {
        gl_loader::get_proc_address(symbol) as *const _
    }
    fn get_framebuffer_dimensions(&self) -> (u32, u32) {
        let allocation = self.glarea.allocation();
        (allocation.width as u32, allocation.height as u32)
    }
    fn is_current(&self) -> bool {
        // GTK makes it current itself on each "render" signal
        true
    }
    unsafe fn make_current(&self) {
        self.glarea.make_current();
    }
}

impl GLAreaBackend {
    fn new(glarea: gtk::GLArea) -> Self {
        Self { glarea }
    }
}

fn main() {
    let application =
        gtk::Application::new(None, Default::default());

    application.connect_activate(|app| {
        let window = gtk::ApplicationWindowBuilder::new()
            .application(app)
            .title("gtk-glium")
            .window_position(gtk::WindowPosition::Center)
            .default_width(600)
            .default_height(400)
            .build();

        let glarea = gtk::GLArea::new();
        window.add(&glarea);
        window.show_all();

        // load gl
        gl_loader::init_gl();

        // create glium context
        let context = unsafe {
            Context::new(
                GLAreaBackend::new(glarea.clone()),
                true,
                glium::debug::DebugCallbackBehavior::DebugMessageOnError,
            )
            .unwrap()
        };

        let facade = Rc::new(&context);

        let renderer = std::rc::Rc::new(Mutex::new(Renderer::new(*facade)));

        glarea.connect_render(move |glarea, _glcontext| {
            let mut frame = glium::Frame::new(context.clone(), context.get_framebuffer_dimensions());
            // this is where you can do your glium rendering

            renderer.lock().unwrap().draw(&mut frame);

            frame.finish().unwrap();

            glarea.queue_draw();

            Inhibit(true)
        });

        // This makes the GLArea redraw 60 times per second
        // You can remove this if you want to redraw only when focused/resized
        // const FPS: u32 = 60;
        // glib::source::timeout_add_local(1_000 / FPS, move || {
        //     glarea.queue_draw();
        //     glib::source::Continue(true)
        // });
    });

    application.run();
}
