pub struct SimulationSettings {
    num_points_x: usize,
    num_points_y: usize,
    dx: f32,
    dy: f32,
    dt: f32,
    c: f32,
}

pub struct RenderSettings {
    num_points_x: usize,
    num_points_y: usize,
    y_scale: f32,
}

pub trait Simulation {
    fn new() -> Self;

    fn simulation_settings(&self) -> SimulationSettings;
    fn render_settings(&self) -> RenderSettings;

    fn psi_0(&self, x: f32, y: f32) -> f32;
}

impl SimulationSettings {
    pub const fn new(
        num_points_x: usize,
        num_points_y: usize,
        dx: f32,
        dy: f32,
        dt: f32,
        c: f32,
    ) -> Self {
        SimulationSettings {
            num_points_x,
            num_points_y,
            dx,
            dy,
            dt,
            c,
        }
    }

    pub fn num_points_x(&self) -> usize {
        self.num_points_x
    }

    pub fn num_points_y(&self) -> usize {
        self.num_points_y
    }

    pub fn dx(&self) -> f32 {
        self.dx
    }

    pub fn dy(&self) -> f32 {
        self.dy
    }

    pub fn dt(&self) -> f32 {
        self.dt
    }

    pub fn c(&self) -> f32 {
        self.c
    }
}

impl RenderSettings {
    pub const fn new(num_points_x: usize, num_points_y: usize, y_scale: f32) -> Self {
        RenderSettings {
            num_points_x,
            num_points_y,
            y_scale,
        }
    }

    pub fn num_points_x(&self) -> usize {
        self.num_points_x
    }

    pub fn num_points_y(&self) -> usize {
        self.num_points_y
    }

    pub fn y_scale(&self) -> f32 {
        self.y_scale
    }
}
