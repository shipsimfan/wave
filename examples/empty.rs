use common::{COMMON_RENDER_SETTINGS, COMMON_SIMULATION_SETTINGS, TIME_SCALE};

mod common;

struct EmptySimulation;

fn main() {
    wave::run::<EmptySimulation>();
}

impl wave::Simulation for EmptySimulation {
    fn new() -> Self {
        EmptySimulation
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

    fn psi_0(&self, _: f32, _: f32) -> (f32, f32) {
        (0.0, 0.0)
    }
}
