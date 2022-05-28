use colosseum::{Input, Vertex, Window};
use std::f32::consts::PI;

enum CurrentWave {
    Wave1,
    Wave2,
    Wave3,
}

#[repr(C)]
struct Settings {
    r: f32,
    color_mod: f32,
    dt: f32,
    dx: f32,
}

pub struct Simulation {
    num_points: usize,
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
    pub fn new<I: Input>(width: f32, dx: f32, dt: f32, r: f32, window: &mut Window<I>) -> Self {
        let num_points = (width / dx).round() as usize + 1;

        assert!(num_points % 64 == 0);

        let shader_code = include_str!("compute.hlsl");
        let compute_shader =
            alexandria::compute::ComputeShader::new(shader_code, window.inner()).unwrap();

        let mut vertices = Vec::with_capacity(num_points);
        let base = -(width / 2.0);
        for i in 0..num_points {
            let x = base + i as f32 * dx;
            let y = if x < -0.15 || x > 0.15 {
                0.0
            } else {
                (10.0 * PI * x).cos() * 0.2
            };

            vertices.push(Vertex::new(x, y, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0));
        }

        let wave1 = alexandria::compute::RWBuffer::new(&vertices, 1, window.inner()).unwrap();
        let wave2 = alexandria::compute::RWBuffer::new(&vertices, 2, window.inner()).unwrap();
        let wave3 = alexandria::compute::RWBuffer::new(&vertices, 0, window.inner()).unwrap();

        println!("R: {}", r);

        let settings = Settings {
            r,
            color_mod: 3.0,
            dt,
            dx,
        };
        let settings = alexandria::ConstantBuffer::new(Some(settings), 0, window.inner()).unwrap();

        Simulation {
            num_points,
            current_wave: CurrentWave::Wave1,
            compute_shader,
            wave1,
            wave2,
            wave3,
            settings,
        }
    }

    pub fn num_points(&self) -> usize {
        self.num_points
    }

    pub fn update<I: Input>(&mut self, window: &mut Window<I>) {
        self.compute_shader.set_active(window.inner());
        self.wave1.set_active(window.inner());
        self.wave2.set_active(window.inner());
        self.wave3.set_active(window.inner());
        self.settings.set_active_compute(window.inner());
        self.compute_shader
            .dispatch(self.num_points / 64, 1, 1, window.inner());

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
