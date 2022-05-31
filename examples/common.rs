#![allow(dead_code)]

pub const NUM_POINTS_X: usize = 256;
pub const NUM_POINTS_Y: usize = NUM_POINTS_X;

pub const DX: f32 = 2.0 / (NUM_POINTS_X as f32 - 1.0);
pub const DY: f32 = DX;

pub const WIDTH: f32 = NUM_POINTS_X as f32 * DX;
pub const HEIGHT: f32 = NUM_POINTS_Y as f32 * DY;

pub const FRAME_TIME: f32 = 1.0 / 60.0;
pub const SUB_STEPS: usize = 1;
pub const DT: f32 = FRAME_TIME / (SUB_STEPS as f32);

pub const C: f32 = 0.05;

pub const Y_SCALE: f32 = 0.25;

pub const COMMON_SIMULATION_SETTINGS: wave::SimulationSettings =
    wave::SimulationSettings::new(NUM_POINTS_X, NUM_POINTS_Y, DX, DY, DT, C);
pub const COMMON_RENDER_SETTINGS: wave::RenderSettings =
    wave::RenderSettings::new(NUM_POINTS_X, NUM_POINTS_Y, Y_SCALE);

fn main() {}
