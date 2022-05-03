use egui::Color32;

const GREEN: Color32 = Color32::from_rgb(100, 200, 100);
const RED: Color32 = Color32::from_rgb(255, 0, 0);

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
#[derive(Debug, Clone)]
pub struct DisplaySettings {
    pub radius: f32,
    pub harmonic_value: i32,
    pub display_colors: Vec<Color32>,
    pub windowed: bool,
    pub boundary_style: String,
    pub width: f64,
}

impl Default for DisplaySettings {
    fn default() -> Self {
        Self {
            radius: 5.0,
            harmonic_value: 1,
            display_colors: vec![RED, GREEN, GREEN],
            windowed: false,
            boundary_style: "line".to_string(),
            width: 1.0,
        }
    }
}
