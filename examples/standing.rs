use common::*;
use std::f32::consts::PI;

mod common;

struct StandingWaveSimulation {
    odd_x: bool,
    odd_y: bool,
    kx: f32,
    ky: f32,
}

const DEFAULT_MODE: usize = 2;

impl wave::Simulation for StandingWaveSimulation {
    fn new() -> Self {
        let mut args = std::env::args();

        let (x_mode, y_mode) = (
            args.next()
                .unwrap_or(format!("{}", DEFAULT_MODE))
                .parse()
                .unwrap_or(DEFAULT_MODE),
            args.next()
                .unwrap_or(format!("{}", DEFAULT_MODE))
                .parse()
                .unwrap_or(DEFAULT_MODE),
        );

        StandingWaveSimulation {
            odd_x: x_mode % 2 == 1,
            odd_y: y_mode % 2 == 1,
            kx: x_mode as f32 * PI / WIDTH,
            ky: y_mode as f32 * PI / HEIGHT,
        }
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
        (
            if self.odd_x {
                (self.kx * x).cos()
            } else {
                (self.kx * x).sin()
            } * if self.odd_y {
                (self.ky * y).cos()
            } else {
                (self.ky * y).sin()
            },
            0.0,
        )
    }
}

fn main() {
    wave::run::<StandingWaveSimulation>()
}
