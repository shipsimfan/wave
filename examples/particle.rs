use common::{COMMON_RENDER_SETTINGS, COMMON_SIMULATION_SETTINGS};
use std::f32::consts::PI;

mod common;

struct ParticleSimulation;

fn main() {
    wave::run::<ParticleSimulation>();
}

impl wave::Simulation for ParticleSimulation {
    fn new() -> Self {
        ParticleSimulation
    }

    fn simulation_settings(&self) -> wave::SimulationSettings {
        COMMON_SIMULATION_SETTINGS
    }

    fn render_settings(&self) -> wave::RenderSettings {
        COMMON_RENDER_SETTINGS
    }

    fn psi_0(&self, x: f32, y: f32) -> f32 {
        1.0 / (75.0 * x * y) * (4.0 * PI * x).sin() * (4.0 * PI * y).sin()
    }
}
