//use std::sync::Arc;
use egui_winit::State as EguiWinitState;
//use egui_glow::Painter;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::WindowBuilder,
};
//use glutin::{
//    config::{ConfigTemplateBuilder, GlConfig},
//    context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext},
//    display::GetGlDisplay,
//    surface::{Surface, SurfaceAttributesBuilder, WindowSurface},
//};
//use glutin_winit::DisplayBuilder;
//use raw_window_handle::HasRawWindowHandle;

use rfd;

// Structure
struct App {
    replay_folder_path: String,
}

impl App {
    fn new() -> Self {
        Self {
            replay_folder_path: String::new(),
        }
    }

    fn ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Choose folder");

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Replay folder path:");
            });

            if ui.button("Choose folder...").clicked() {
                if let Some(path_buff) = rfd::FileDialog::new().pick_folder() {
                    self.replay_folder_path = path_buff.display().to_string();
                    println!("Chosen folder: {}", self.replay_folder_path);
                    // Logic :3
                } else {
                    println!("No chosen folder...");
                }
            }
        });
    }
}

fn main() {
    // Create Event Loop
    let event_loop = EventLoop::new().expect("Error creating event loop");

    // WINDOW
    let window = WindowBuilder::new()
        .with_title("Test")
        .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0))
        .build(&event_loop)
        .expect("Error creating window");

    println!("Window created!");

    let mut egui_ctx = egui::Context::default();
    let mut egui_winit_state = EguiWinitState::new(
        egui_ctx.clone(),
        egui_ctx.viewport_id(),
        &window,
        None, // Scale Factor
        None, // Max Texture Scale
    );

    let mut app = App::new();

    // EVENTS
    // Start Event Loop
    event_loop.run(move |event, elwt| {

        // Wait for Event
        elwt.set_control_flow(ControlFlow::Wait);

        // EVENT!!!!!!!!!!
        match event {
            // Close Window
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                elwt.exit();
            }
            // Pass window events to egui_winit_state
            Event::WindowEvent {
                event,
                window_id,
            } if window_id == window.id() => {
                egui_winit_state.on_window_event(&window, &event);
                window.request_redraw();
            }

            // New Frame (FUCK IT!!!!!)
            Event::AboutToWait => {
                // Begin new egui frame
                let raw_input = egui_winit_state.take_egui_input(&window);
                let full_output = egui_ctx.run(raw_input, |ctx| {
                    app.ui(ctx);
                });

                egui_winit_state.handle_platform_output(&window, full_output.platform_output);
                window.request_redraw();
            }

            // Ignore other events!!!!!
            _ => (),
        }
    }).expect("Error eventloop run");
}