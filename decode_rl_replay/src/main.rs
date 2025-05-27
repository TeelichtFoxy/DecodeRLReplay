// Window Inputs
use egui::{CentralPanel, TextEdit, TopBottomPanel, ScrollArea};
use egui_glium::EguiGlium;
use glium::{
    glutin::{self, surface::WindowSurface},
    Surface,
};
// Window
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new()
        .with_title("Decode RL Replay")
        .with_inner_size(winit::dpi::PhysicalSize::new(800, 600));
    
    let display = glium::Display::new(window_builder, &event_loop).unwrap();

    let mut egui_glium = EguiGlium::new(&display, &event_loop);

    let mut replay_folder_path = String::from("");

    event_loop.run(move |event, elwt| {
        egui_glium.on_event(&event);

        elwt.set_control_flow(ControlFlow::Wait);

        match event {
            // Close Window
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == display.gl_window().window().id() => {
                println!("Close Requested! Exiting.");
                elwt.exit();
            }
            // About To Wait
            Event::AboutToWait => {
                // Redraw Window
                display.gl_window().window().request_redraw();
            }
            // Redraw Window
            Event::RedrawRequested(window_id) if window_id == display.gl_window().window().id() => {
                egui_glium.begin_frame();

                CentralPanel::default().show(egui_glium.egui.ctx(), |ui| {
                    ui.heading("Decode RL Replay");
                    ui.seperator();

                    ui.label("Replay Folder:");
                    ui.add(
                        TextEdit::singleline(&mut replay_folder_path)
                            .hint_text("Path to the folder for the .replay files"),
                    );
                })
            }
        }
    })
}