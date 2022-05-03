use std::f64::consts::PI;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
#[derive(Debug, Clone, Copy)]
struct Mass {
    pos: f64,
    past_pos: f64,
    accel: f64,
}
impl Default for Mass {
    fn default() -> Self {
        Self {
            pos: 0.0,
            past_pos: 0.0,
            accel: 0.0,
        }
    }
}
impl Mass {
    fn new(p: f64) -> Mass {
        Mass {
            pos: p,
            past_pos: p,
            accel: 0.0,
        }
    }

    fn update_position(&mut self, t: f64, delta: f64) {
        if t == 0.0 {
            self.past_pos = self.pos;
        } else if t == delta {
            self.pos = self.past_pos + 0.5 * self.accel * square(delta);
        } else {
            let cur = self.pos;
            self.pos = 2.0 * cur - self.past_pos + self.accel * square(delta);
            self.past_pos = cur;
        }
    }

    fn update_acceleration(&mut self, l_pos: f64, r_pos: f64) {
        self.accel = l_pos - 2.0 * self.pos + r_pos;
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
#[derive(Debug, Clone)]
pub struct Sys {
    masses: Vec<Mass>,
}

impl Sys {
    pub fn new(m: usize, size: usize, displacement: f64) -> Sys {
        let mut new_system = Sys {
            masses: vec![Mass::new(0.0); size],
        };
        new_system.masses[m].pos = displacement;
        new_system
    }

    pub fn push(&mut self, displacement: f64) {
        self.masses.push(Mass::new(displacement));
    }

    pub fn pop(&mut self) {
        self.masses.pop();
    }

    pub fn update_system(&mut self, time_step: &mut f64, delta: f64) {
        for i in 0..self.masses.len() {
            self.masses[i].update_position(*time_step, delta);
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
        *time_step += delta;
    }

    pub fn get_mass_pos(&self, mass: usize) -> f64 {
        self.masses[mass].pos
    }

    pub fn len(&self) -> usize {
        self.masses.len()
    }

    pub fn parabola(&mut self, height: f64) {
        let base = if self.masses.len() % 2 == 0 { 0.5 } else { 0.0 };
        let spacing = (self.masses.len() + (self.masses.len() % 2)) as f64;
        for i in 0..self.masses.len() {
            let pos =
                -1.0 / square(spacing / 2.0) * square(i as f64 + 1.0 - base - spacing / 2.0) + 1.0;
            self.masses[i] = Mass::new(pos * height);
        }
    }
    pub fn harmonic_state(&mut self, height: f64, state: i32) {
        let spacing = (self.masses.len() + 1) as f64;
        for i in 0..self.masses.len() {
            let pos = ((i + 1) as f64 / (spacing) * PI * (state as f64)).sin();
            self.masses[i] = Mass::new(pos * height);
        }
    }

    pub fn pluck(&mut self, height: f64) {
        let n = self.masses.len();
        for i in 0..n {
            let pos = -2.0 * f64::abs(((i + 1) as f64 / (n as f64 + 1.0)) - 0.5) + 1.0;

            self.masses[i] = Mass::new(pos * height);
        }
    }

    pub fn alter(&mut self, i: usize, displacement: f64) {
        self.masses[i].pos = displacement;
    }
}

impl Default for Sys {
    fn default() -> Self {
        Self {
            masses: vec![Mass::new(1.0)],
        }
    }
}

pub fn square(val: f64) -> f64 {
    val * val
}
pub fn round(val: f64, rounding_factor: f64) -> f64 {
    f64::floor(val / rounding_factor) * rounding_factor
}
