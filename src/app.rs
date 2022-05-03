mod display;
mod string_dynamics;

use self::display::DisplaySettings;
use self::string_dynamics::*;
use eframe::egui;
use eframe::egui::{plot::*, Ui};
use eframe::epaint::Color32;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
#[derive(Debug, Clone)]
pub struct SystemPlot {
    animate: bool,
    time: f64,
    system: string_dynamics::Sys,
    size: usize,
    initial_displacement: f64,
    clamped: bool,
    max_time: f64,
    delta: f64,
    display_settings: DisplaySettings,
}

impl Default for SystemPlot {
    fn default() -> Self {
        Self {
            animate: false,
            time: 0.0,
            system: Sys::new(0, 10, 1.0),
            size: 10,
            initial_displacement: 1.0,
            clamped: false,
            max_time: 100.0,
            delta: 0.1,
            display_settings: Default::default(),
        }
    }
}

impl SystemPlot {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn line_points(&self, width: f64) -> Line {
        let n = self.size;
        let points = (0..n + 2).map(|i| {
            if i == 0 || i == n + 1 {
                Value::new(((i as f64) / ((n + 1) as f64)) * width, 0.0)
            } else {
                Value::new(
                    ((i as f64) / ((n + 1) as f64)) * width,
                    self.system.get_mass_pos(i - 1),
                )
            }
        });

        Line::new(Values::from_values_iter(points))
            .style(LineStyle::Solid)
            .name("mass")
    }

    fn circle_points(&self, radius: f32, width: f64) -> Points {
        let n = self.size;
        let circle = (0..n).map(|i| {
            Value::new(
                ((i as f64 + 1.0) / ((n + 1) as f64)) * width,
                self.system.get_mass_pos(i),
            )
        });
        Points::new(Values::from_values_iter(circle))
            .name("mass")
            .filled(true)
            .radius(radius)
            .shape(MarkerShape::Circle)
    }

    fn display(&mut self, ui: &mut Ui) {
        ui.heading("Side Panel");
        ui.label(format!("Time: {:.1}", self.time));
        egui::CollapsingHeader::new("Basic Settings").show(ui, |ui| {
            if ui.button("Reset").clicked() {
                self.animate = false;
                self.system = Sys::new(0, 10, 1.0);
                self.size = 10;
                self.time = 0.0;
            }
            ui.add(egui::Slider::new(&mut self.size, 1..=500).text("Masses"));
            ui.horizontal(|ui| {
                if ui.button("Increment").clicked() && self.size < 500 {
                    self.size += 1;
                }
                if ui.button("Decrement").clicked() && self.size > 1 {
                    self.size -= 1;
                }
            });
            let time_response = ui.add(
                egui::DragValue::new(&mut self.delta)
                    .clamp_range(0.001f64..=0.750f64)
                    .speed(0.001)
                    .prefix("Delta: "),
            );
            if time_response.changed() {
                self.animate = false;
                self.system = Sys::new(0, self.size, 1.0);
                self.time = 0.0;
            }

            ui.checkbox(&mut self.animate, "Animate");
            if ui.button("Step").clicked() {
                self.animate = false;
                self.system.update_system(&mut self.time, self.delta);
            }
        });

        while self.size > self.system.len() {
            self.system.push(0.0);
        }
        while self.size < self.system.len() {
            self.system.pop();
        }
        egui::CollapsingHeader::new("Display Settings").show(ui, |ui| {
            ui.add(
                egui::DragValue::new(&mut self.display_settings.radius)
                    .speed(0.1)
                    .clamp_range(0.0f64..=100.0f64)
                    .prefix("Mass radius (display): "),
            );
            ui.add(
                egui::DragValue::new(&mut self.display_settings.width)
                    .speed(0.1)
                    .clamp_range(0.5f64..=100.0f64)
                    .prefix("String length (display): "),
            );

            ui.horizontal(|ui| {
                ui.label("Mass Color:");
                ui.color_edit_button_srgba(&mut self.display_settings.display_colors[2]);
            });
            ui.horizontal(|ui| {
                ui.label("String Color:");
                ui.color_edit_button_srgba(&mut self.display_settings.display_colors[1]);
            });
            ui.horizontal(|ui| {
                ui.label("Boundary Color:");
                ui.color_edit_button_srgba(&mut self.display_settings.display_colors[0]);
            });
            egui::ComboBox::from_label("Boundary style")
                .selected_text(self.display_settings.boundary_style.to_string())
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.display_settings.boundary_style,
                        "line".to_string(),
                        "Lines",
                    );
                    ui.selectable_value(
                        &mut self.display_settings.boundary_style,
                        "mass".to_string(),
                        "Masses",
                    );
                    ui.selectable_value(
                        &mut self.display_settings.boundary_style,
                        "none".to_string(),
                        "None",
                    );
                });

            if ui.button("Windowed").clicked() {
                self.display_settings.windowed = !self.display_settings.windowed;
            }
        });
        egui::CollapsingHeader::new("Misc State Settings").show(ui, |ui| {
            ui.add(
                egui::DragValue::new(&mut self.initial_displacement)
                    .speed(0.1)
                    .clamp_range(0.0..=f64::INFINITY)
                    .prefix("Initial Displacement: "),
            );
            let harmonic_response = ui.add(
                egui::DragValue::new(&mut self.display_settings.harmonic_value)
                    .speed(0.01)
                    .clamp_range(1i32..=8i32)
                    .prefix("Harmonic State: "),
            );
            if harmonic_response.changed() {
                self.animate = false;
                self.system = Sys::new(0, self.size, 0.0);
                self.time = 0.0;
                self.system.harmonic_state(
                    self.initial_displacement,
                    self.display_settings.harmonic_value,
                );
            }
            ui.separator();
            if ui.button("Harmonic").clicked() {
                self.animate = false;
                self.system = Sys::new(0, self.size, 0.0);
                self.time = 0.0;
                self.system.harmonic_state(
                    self.initial_displacement,
                    self.display_settings.harmonic_value,
                );
            }
            if ui.button("Parabolic").clicked() {
                self.animate = false;
                self.system = Sys::new(0, self.size, 0.0);
                self.time = 0.0;
                self.system.parabola(self.initial_displacement);
            }
            if ui.button("Pluck").clicked() {
                self.animate = false;
                self.system = Sys::new(0, self.size, 0.0);
                self.time = 0.0;
                self.system.pluck(self.initial_displacement);
            }
        });

        egui::CollapsingHeader::new("Clamped Settings").show(ui, |ui| {
            ui.add(
                egui::DragValue::new(&mut self.max_time)
                    .speed(0.1)
                    .clamp_range(0.0..=f64::INFINITY)
                    .prefix("Max Time: "),
            );
            ui.checkbox(&mut self.clamped, "Clamped");
        });

        egui::CollapsingHeader::new("Mass positions").show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for i in 0..self.size {
                    let mut x = self.system.get_mass_pos(i);
                    let mass_response =
                        ui.add(egui::Slider::new(&mut x, -10.0..=10.0).text(format!("{}: ", i)));
                    if mass_response.changed() {
                        self.system.alter(i, x);
                    }
                }
            })
        });
        egui::warn_if_debug_build(ui);
    }
}

impl eframe::App for SystemPlot {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self {
            animate: _,
            time: _,
            system: _,
            size: _,
            ..
        } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                    if ui.button("Toggle Window").clicked() {
                        self.display_settings.windowed = !self.display_settings.windowed;
                    }
                    if ui.button("Full Reset").clicked() {
                        *self = Default::default();
                    }
                });
            });
        });
        if !self.display_settings.windowed {
            egui::SidePanel::left("side_panel").show(ctx, |ui| {
                self.display(ui);
            });
        } else {
            egui::Window::new("Settings").show(ctx, |ui| {
                self.display(ui);
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.ctx().request_repaint();

            let plot = Plot::new("Loaded String")
                .legend(Legend::default())
                .data_aspect(1.0);

            plot.show(ui, |plot_ui| {
                plot_ui.line(
                    self.line_points(self.display_settings.width)
                        .color(self.display_settings.display_colors[1]),
                );
                plot_ui.points(
                    self.circle_points(self.display_settings.radius, self.display_settings.width)
                        .color(self.display_settings.display_colors[2]),
                );
                if self.display_settings.display_colors[0] != Color32::TRANSPARENT {
                    if self.display_settings.boundary_style == "line" {
                        plot_ui
                            .vline(VLine::new(0.0).color(self.display_settings.display_colors[0]));
                        plot_ui.vline(
                            VLine::new(self.display_settings.width)
                                .color(self.display_settings.display_colors[0]),
                        );
                    } else if self.display_settings.boundary_style == "mass" {
                        plot_ui.points(
                            Points::new(Values::from_values(vec![
                                Value::new(0.0, 0.0),
                                Value::new(self.display_settings.width, 0.0),
                            ]))
                            .color(self.display_settings.display_colors[0])
                            .radius(4.0),
                        );
                    }
                }
            });
        });

        if self.animate {
            if round(self.time, self.delta) >= self.max_time && self.clamped {
                self.animate = false;
            } else {
                self.system.update_system(&mut self.time, self.delta);
            }
        }
    }
}
