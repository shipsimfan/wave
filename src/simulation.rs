use std::f32::consts::PI;

use colosseum::{Input, Vertex, Window};

enum CurrentWave {
    Wave1,
    Wave2,
    Wave3,
}

#[repr(C)]
struct Settings {
    r: f32,
    dx: f32,
    dy: f32,
    dt: f32,

    color_mod: f32,
    num_points_x: u32,
    num_points_y: u32,
    reserved: f32,
}

pub struct Simulation {
    num_points_x: usize,
    num_points_y: usize,
    width: f32,
    height: f32,
    dx: f32,
    dy: f32,
    current_wave: CurrentWave,

    // Wave buffers
    wave1: alexandria::compute::RWBuffer<Vertex>,
    wave2: alexandria::compute::RWBuffer<Vertex>,
    wave3: alexandria::compute::RWBuffer<Vertex>,

    // Constant buffer
    settings: alexandria::ConstantBuffer<Settings>,

    // Compute shader
    compute_shader: alexandria::compute::ComputeShader,
}

impl Simulation {
    pub fn new<I: Input>(
        num_points_x: usize,
        dx: f32,
        num_points_y: usize,
        dy: f32,
        dt: f32,
        c: f32,
        window: &mut Window<I>,
    ) -> Self {
        assert_eq!(num_points_x % 16, 0);
        assert_eq!(num_points_y % 16, 0);

        let width = ((num_points_x - 1) as f32) * dx;
        let height = ((num_points_y - 1) as f32) * dy;

        let shader_code = include_str!("compute.hlsl");
        let compute_shader =
            alexandria::compute::ComputeShader::new(shader_code, window.inner()).unwrap();

        let mut vertices = Vec::with_capacity(num_points_x * num_points_y);
        let base_x = -(width / 2.0);
        let base_y = -(height / 2.0);

        const WAVE_COUNT: usize = 3;
        let _kx = WAVE_COUNT as f32 * PI / width;
        let _ky = WAVE_COUNT as f32 * PI / height;

        let inv_pi_sqrt = 1.0 / PI.sqrt();

        for y in 0..num_points_y {
            for x in 0..num_points_x {
                let x = base_x + x as f32 * dx;
                let z = base_y + y as f32 * dy;

                // Single "particle" wave
                let y = inv_pi_sqrt * 1.0 / x * x.sin() * 0.5;

                // Standing wave
                /*let y = if WAVE_COUNT % 2 == 0 {
                    (_kx * x).sin() * (_ky * z).sin()
                } else {
                    (_kx * x).cos() * (_ky * z).cos()
                } * 0.25;*/

                vertices.push(Vertex::new(x, y, z, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0));
            }
        }

        let wave1 = alexandria::compute::RWBuffer::new(&vertices, 1, window.inner()).unwrap();
        let wave2 = alexandria::compute::RWBuffer::new(&vertices, 2, window.inner()).unwrap();
        let wave3 = alexandria::compute::RWBuffer::new(&vertices, 0, window.inner()).unwrap();

        let settings = Settings {
            r: c * c * dt * dt,
            dx,
            dy,
            dt,
            color_mod: 3.0,
            num_points_x: num_points_x as u32,
            num_points_y: num_points_y as u32,
            reserved: 0.0,
        };
        let settings = alexandria::ConstantBuffer::new(Some(settings), 0, window.inner()).unwrap();

        Simulation {
            num_points_x,
            num_points_y,
            width,
            height,
            dx,
            dy,
            current_wave: CurrentWave::Wave1,
            compute_shader,
            wave1,
            wave2,
            wave3,
            settings,
        }
    }

    pub fn num_points_x(&self) -> usize {
        self.num_points_x
    }

    pub fn num_points_y(&self) -> usize {
        self.num_points_y
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn dx(&self) -> f32 {
        self.dx
    }

    pub fn dy(&self) -> f32 {
        self.dy
    }

    pub fn update<I: Input>(&mut self, window: &mut Window<I>) {
        self.compute_shader.set_active(window.inner());
        self.wave1.set_active(window.inner());
        self.wave2.set_active(window.inner());
        self.wave3.set_active(window.inner());
        self.settings.set_active_compute(window.inner());
        self.compute_shader.dispatch(
            self.num_points_x / 16,
            self.num_points_y / 16,
            1,
            window.inner(),
        );

        self.set_next_wave()
    }

    pub fn current_wave(&mut self) -> &mut alexandria::compute::RWBuffer<Vertex> {
        match self.current_wave {
            CurrentWave::Wave1 => &mut self.wave1,
            CurrentWave::Wave2 => &mut self.wave2,
            CurrentWave::Wave3 => &mut self.wave3,
        }
    }

    fn set_next_wave(&mut self) {
        self.current_wave = match self.current_wave {
            CurrentWave::Wave1 => {
                self.wave1.set_slot(1);
                self.wave2.set_slot(2);
                self.wave3.set_slot(0);
                CurrentWave::Wave2
            }
            CurrentWave::Wave2 => {
                self.wave1.set_slot(0);
                self.wave2.set_slot(1);
                self.wave3.set_slot(2);
                CurrentWave::Wave3
            }
            CurrentWave::Wave3 => {
                self.wave1.set_slot(2);
                self.wave2.set_slot(0);
                self.wave3.set_slot(1);
                CurrentWave::Wave1
            }
        };
    }
}
