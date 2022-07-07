#![allow(dead_code)]

pub const NUM_POINTS_X: usize = 256;
pub const NUM_POINTS_Y: usize = NUM_POINTS_X;

pub const DX: f32 = 1e-10;
pub const DY: f32 = DX;

pub const WIDTH: f32 = NUM_POINTS_X as f32 * DX;
pub const HEIGHT: f32 = NUM_POINTS_Y as f32 * DY;

pub const DT: f32 = 5.39e-11;

pub const MASS: f32 = 9.1093837e-31;

pub const Y_SCALE: f32 = 0.25;
pub const XZ_SCALE: f32 = 1.0 / (DX * NUM_POINTS_X as f32);

pub const SUB_STEPS: usize = 1;
pub const TIME_SCALE: f32 = 1.0 / (60.0 * DT * SUB_STEPS as f32);

pub const COMMON_SIMULATION_SETTINGS: wave::SimulationSettings =
    wave::SimulationSettings::new(NUM_POINTS_X, NUM_POINTS_Y, DX, DY, DT, MASS);
pub const COMMON_RENDER_SETTINGS: wave::RenderSettings =
    wave::RenderSettings::new(NUM_POINTS_X, NUM_POINTS_Y, Y_SCALE, XZ_SCALE);

fn main() {}
