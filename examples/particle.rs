use common::*;

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

    fn time_scale(&self) -> f32 {
        TIME_SCALE
    }

    fn psi_0(&self, x: f32, y: f32) -> (f32, f32) {
        (1.0 - x * XZ_SCALE, 0.0)
    }
}
