use eframe::egui::plot::*;
use eframe::epaint::Color32;
use eframe::{egui, epi};

const DELTA: f64 = 0.1;
const DELTA_SQUARE: f64 = 0.01;
const RED: Color32 = Color32::from_rgb(255, 0, 0);
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
#[derive(Debug, Clone, Copy)]
struct Mass {
    pos: f64,
    past_pos: f64,
    accel: f64,
}

impl Mass {
    fn new(p: f64) -> Mass {
        return Mass {
            pos: p,
            past_pos: p,
            accel: 0.0,
        };
    }

    fn update_position(&mut self, t: f64) {
        if t == 0.0 {
            self.past_pos = self.pos;
        } else if t == DELTA {
            self.pos = self.past_pos + 0.5 * self.accel * DELTA_SQUARE;
        } else {
            let cur = self.pos;
            self.pos = 2.0 * cur - self.past_pos + self.accel * DELTA_SQUARE;
            self.past_pos = cur;
        }
        //if f64::abs(self.pos) > 1.0 { println!("Mass exceeded initial displacement"); }
    }

    fn update_acceleration(&mut self, l_pos: f64, r_pos: f64) {
        self.accel = l_pos - 2.0 * self.pos + r_pos;
    }
}

#[derive(Debug, Clone)]
struct Sys {
    masses: Vec<Mass>,
}

impl Sys {
    fn new(m: usize, size: usize, displacement: f64) -> Sys {
        let mut new_system = Sys {
            masses: vec![Mass::new(0.0); size],
        };
        new_system.masses[m].pos = displacement;
        return new_system;
    }

    fn update_system(&mut self, time_step: f64) {
        for i in 0..self.masses.len() {
            self.masses[i].update_position(time_step);
        }
        for i in 0..self.masses.len() {
            if i == 0 {
                if self.masses.len() > 1 {
                    let r = self.masses[i + 1].pos;
                    self.masses[i].update_acceleration(0.0, r);
                } else {
                    self.masses[i].update_acceleration(0.0, 0.0);
                }
            } else if i == self.masses.len() - 1 {
                let l = self.masses[i - 1].pos;
                self.masses[i].update_acceleration(l, 0.0);
            } else {
                let r = self.masses[i + 1].pos;
                let l = self.masses[i - 1].pos;
                self.masses[i].update_acceleration(l, r);
            }
        }
    }

    fn harmonic_state(&mut self, height: f64) {
        let base = if self.masses.len() % 2 == 0 { 0.5 } else { 0.0 };
        let spacing = (self.masses.len() + (self.masses.len() % 2)) as f64;
        for i in 0..self.masses.len() {
            let pos =
                -1.0 / square(spacing / 2.0) * square(i as f64 + 1.0 - base - spacing / 2.0) + 1.0;
            self.masses[i] = Mass::new(pos * height);
        }
    }
}

fn square(val: f64) -> f64 {
    val * val
}

pub struct SystemPlot {
    animate: bool,
    time: f64,
    system: Sys,
    size: usize,
    radius: f32,
    initial_displacement: f64,
    clamped: bool,
    max_time: f64,
}

impl Default for SystemPlot {
    fn default() -> Self {
        Self {
            animate: false,
            time: 0.0,
            system: Sys::new(0, 10, 1.0),
            size: 10,
            radius: 5.0,
            initial_displacement: 1.0,
            clamped: false,
            max_time: 100.0,
        }
    }
}

impl SystemPlot {
    fn line_points(&self) -> Line {
        let n = self.size;
        let points = (0..n + 2).map(|i| {
            if i == 0 || i == n + 1 {
                Value::new((i as f64) / ((n + 1) as f64), 0.0)
            } else {
                Value::new((i as f64) / ((n + 1) as f64), self.system.masses[i - 1].pos)
            }
        });

        Line::new(Values::from_values_iter(points))
            .color(Color32::from_rgb(100, 200, 100))
            .style(LineStyle::Solid)
            .name("mass")
    }

    fn circle_points(&self, radius: f32) -> Points {
        let marker = MarkerShape::Circle;
        let n = self.size;
        let circle = (0..n).map(|i| {
            Value::new(
                (i as f64 + 1.0) / ((n + 1) as f64),
                self.system.masses[i].pos,
            )
        });
        Points::new(Values::from_values_iter(circle))
            .name("mass")
            .filled(true)
            .radius(radius)
            .shape(marker)
            .color(Color32::from_rgb(100, 200, 100))
    }
}

impl epi::App for SystemPlot {
    fn name(&self) -> &str {
        "Loaded String Simulation"
    }

    // Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        let Self {
            animate: _,
            time: _,
            system,
            size,
            ..
        } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");
            ui.label(format!("Time: {:.1}", self.time));

            if ui.button("Reset").clicked() {
                self.animate = false;
                *system = Sys::new(0, 10, 1.0);
                *size = 10;
                self.time = 0.0;
            }

            ui.add(egui::Slider::new(size, 1..=300).text("Masses"));
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.spacing_mut().item_spacing.y = 0.0;
                if ui.button("Increment").clicked() && *size < 300 {
                    *size += 1;
                }
                if ui.button("Decrement").clicked() && *size > 1 {
                    *size -= 1;
                }
            });
            while *size > system.masses.len() {
                system.masses.push(Mass::new(0.0));
            }
            while *size < system.masses.len() {
                system.masses.pop();
            }

            let popup_id = ui.make_persistent_id("harmonic_popup");
            let response = ui.button("Harmonic state");
            if response.clicked() {
                self.animate = false;
                self.time = 0.0;
                ui.memory().toggle_popup(popup_id);
            }
            egui::popup::popup_below_widget(ui, popup_id, &response, |ui| {
                ui.set_min_width(200.0); // if you want to control the size
                ui.add(
                    egui::DragValue::new(&mut self.initial_displacement)
                        .speed(0.1)
                        .clamp_range(0.0..=f64::INFINITY)
                        .prefix("Initial Displacement: "),
                );
                if ui.button("Start").clicked() {
                    system.harmonic_state(self.initial_displacement);
                }
            });

            let time_popup = ui.make_persistent_id("max_time_popup");
            let time_response = ui.button("Clamped");
            if time_response.clicked() {
                self.animate = false;
                self.time = 0.0;
                ui.memory().toggle_popup(time_popup);
            }
            egui::popup::popup_below_widget(ui, time_popup, &time_response, |ui| {
                ui.set_min_width(200.0); // if you want to control the size
                ui.add(
                    egui::DragValue::new(&mut self.max_time)
                        .speed(0.1)
                        .clamp_range(0.0..=f64::INFINITY)
                        .prefix("Max Time: "),
                );
                let t = if self.clamped {
                    "disable".to_string()
                } else {
                    "enable".to_string()
                };
                if ui.button(t).clicked() {
                    self.clamped = !self.clamped;
                }
            });
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.spacing_mut().item_spacing.y = 0.0;
                if ui.button("Animate").clicked() {
                    self.animate = !self.animate;
                }
                if ui.button("Step").clicked() {
                    self.animate = false;
                    system.update_system(self.time);
                    self.time += DELTA;
                }
            });
            ui.add(
                egui::DragValue::new(&mut self.radius)
                    .speed(0.1)
                    .clamp_range(0.0..=f64::INFINITY)
                    .prefix("Mass radius (display): "),
            );

            egui::ScrollArea::vertical().show(ui, |ui| {
                for i in 0..*size {
                    ui.add(
                        egui::Slider::new(&mut system.masses[i].pos, -10.0..=10.0)
                            .text(format!("{}: ", i)),
                    );
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.animate {
                ui.ctx().request_repaint();
            };
            let plot = Plot::new("Loaded String")
                .legend(Legend::default())
                .view_aspect(1.0)
                .data_aspect(1.0);
            //.center_x_axis(true)
            //.center_y_axis(true);

            plot.show(ui, |plot_ui| {
                plot_ui.line(self.line_points());

                plot_ui.points(self.circle_points(self.radius));
                plot_ui.vline(VLine::new(0).color(RED));
                plot_ui.vline(VLine::new(1).color(RED));
            });

            egui::warn_if_debug_build(ui);
        });
        if self.animate {
            if self.time >= self.max_time && self.clamped {
                self.animate = false;
            } else {
                self.system.update_system(self.time);
                self.time += DELTA;
            }
        }
    }
}
