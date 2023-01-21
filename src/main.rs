use imgui::*;

mod window;

fn main() {
    initialize_window();
}

fn initialize_window() {
    let system = window::init(file!());

    let mut show_text = false;
    let mut input = String::new();

    let display = system.display.clone();

    system.main_loop(move |_, ui| {
        let display_dimensions = display.get_framebuffer_dimensions();
        let display_width = display_dimensions.0 as f32;
        let display_height = display_dimensions.1 as f32;

        ui.window("Window with Rust and Dear ImGui!")
            .position([0.0, 0.0], Condition::Always)
            .size([display_width, display_height], Condition::Always)
            .resizable(false)
            .movable(false)
            .build(|| {
                // when resizing main window, additional frames are pushed,
                // which causes our counter to be overly optimistic
                let time_elapsed = (ui.time() * 100.0).round() / 100.0;
                let frame_count = ui.frame_count();
                let fps = (frame_count as f64 / time_elapsed).round();

                ui.text(format!("FPS: {}", fps.to_string()));
                ui.separator();

                ui.checkbox("Say hello", &mut show_text);
                if show_text {
                    ui.text("Hello!");
                }
                ui.separator();

                // why need to .build()?
                ui.input_text("Say something", &mut input).build();
                ui.text(format!("You said: {}", input));
                ui.separator();

                let viewport = display.get_framebuffer_dimensions();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!("Window size{}, {}", viewport.0, viewport.1));
                ui.text(format!(
                    "Cursor [X, Y]: [{:.1}, {:.1}]",
                    mouse_pos[0], mouse_pos[1]
                ));
            });
    });
}
