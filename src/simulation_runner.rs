use crate::Simulation;
use colosseum::{Input, Window};

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

pub struct SimulationRunner {
    num_thread_groups_x: usize,
    num_thread_groups_y: usize,
    width: f32,
    height: f32,
    dx: f32,
    dy: f32,
    dt: f32,
    current_wave: CurrentWave,

    // Wave buffers
    wave1: alexandria::compute::Buffer<f32>,
    wave2: alexandria::compute::Buffer<f32>,
    wave3: alexandria::compute::Buffer<f32>,

    output: alexandria::Texture,

    // Constant buffer
    settings_buffer: alexandria::ConstantBuffer<Settings>,

    // Compute shader
    compute_shader: alexandria::compute::ComputeShader,
}

const PREVIOUS_WAVE_SLOT: usize = 0;
const CURRENT_WAVE_SLOT: usize = 1;
const NEXT_WAVE_SLOT: usize = 2;
const OUTPUT_SLOT: usize = 3;

impl SimulationRunner {
    pub fn new<I: Input, S: Simulation>(simulation: &S, window: &mut Window<I>) -> Self {
        let settings = simulation.simulation_settings();

        assert_eq!(settings.num_points_x() % 16, 0);
        assert_eq!(settings.num_points_y() % 16, 0);

        let width = ((settings.num_points_x() - 1) as f32) * settings.dx();
        let height = ((settings.num_points_y() - 1) as f32) * settings.dy();

        let shader_code = include_str!("compute.hlsl");
        let compute_shader =
            alexandria::compute::ComputeShader::new(shader_code, window.inner()).unwrap();

        let mut values = Vec::with_capacity(settings.num_points_x() * settings.num_points_y());
        let base_x = -(width / 2.0);
        let base_y = -(height / 2.0);
        for y in 0..settings.num_points_y() {
            for x in 0..settings.num_points_x() {
                let x = base_x + x as f32 * settings.dx();
                let y = base_y + y as f32 * settings.dy();

                values.push(simulation.psi_0(x, y));
            }
        }

        let wave1 =
            alexandria::compute::Buffer::new(&values, CURRENT_WAVE_SLOT, window.inner()).unwrap();
        let wave2 =
            alexandria::compute::Buffer::new(&values, NEXT_WAVE_SLOT, window.inner()).unwrap();
        let wave3 =
            alexandria::compute::Buffer::new(&values, PREVIOUS_WAVE_SLOT, window.inner()).unwrap();

        let output = alexandria::Texture::new_1f(
            &values,
            settings.num_points_x(),
            OUTPUT_SLOT,
            window.inner(),
        );

        let settings_buffer = Settings {
            r: settings.c() * settings.c() * settings.dt() * settings.dt(),
            dx: settings.dx(),
            dy: settings.dy(),
            dt: settings.dt(),
            color_mod: 3.0,
            num_points_x: settings.num_points_x() as u32,
            num_points_y: settings.num_points_y() as u32,
            reserved: 0.0,
        };
        let settings_buffer =
            alexandria::ConstantBuffer::new(Some(settings_buffer), 0, window.inner()).unwrap();

        SimulationRunner {
            num_thread_groups_x: settings.num_points_x() / 16,
            num_thread_groups_y: settings.num_points_y() / 16,
            width,
            height,
            dx: settings.dx(),
            dy: settings.dy(),
            dt: settings.dt(),
            current_wave: CurrentWave::Wave1,
            compute_shader,
            wave1,
            wave2,
            wave3,
            output,
            settings_buffer,
        }
    }

    pub fn num_points_x(&self) -> usize {
        self.num_thread_groups_x * 16
    }

    pub fn num_points_y(&self) -> usize {
        self.num_thread_groups_y * 16
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

    pub fn dt(&self) -> f32 {
        self.dt
    }

    pub fn update<I: Input>(&mut self, window: &mut Window<I>) {
        self.previous_wave().set_slot(PREVIOUS_WAVE_SLOT);
        self.current_wave().set_slot(CURRENT_WAVE_SLOT);
        self.next_wave().set_slot(NEXT_WAVE_SLOT);

        self.compute_shader.set_active(window.inner());
        self.previous_wave().set_active_rw(window.inner());
        self.current_wave().set_active_rw(window.inner());
        self.next_wave().set_active_rw(window.inner());
        self.output.set_active_compute_rw(window.inner());
        self.settings_buffer.set_active_compute(window.inner());

        self.compute_shader.dispatch(
            self.num_thread_groups_x,
            self.num_thread_groups_y,
            1,
            window.inner(),
        );

        self.set_next_wave();
    }

    pub fn output(&mut self) -> &mut alexandria::Texture {
        &mut self.output
    }

    fn current_wave(&mut self) -> &mut alexandria::compute::Buffer<f32> {
        match self.current_wave {
            CurrentWave::Wave1 => &mut self.wave1,
            CurrentWave::Wave2 => &mut self.wave2,
            CurrentWave::Wave3 => &mut self.wave3,
        }
    }

    fn next_wave(&mut self) -> &mut alexandria::compute::Buffer<f32> {
        match self.current_wave {
            CurrentWave::Wave1 => &mut self.wave2,
            CurrentWave::Wave2 => &mut self.wave3,
            CurrentWave::Wave3 => &mut self.wave1,
        }
    }

    fn previous_wave(&mut self) -> &mut alexandria::compute::Buffer<f32> {
        match self.current_wave {
            CurrentWave::Wave1 => &mut self.wave3,
            CurrentWave::Wave2 => &mut self.wave1,
            CurrentWave::Wave3 => &mut self.wave2,
        }
    }

    fn set_next_wave(&mut self) {
        self.current_wave = match self.current_wave {
            CurrentWave::Wave1 => CurrentWave::Wave2,
            CurrentWave::Wave2 => CurrentWave::Wave3,
            CurrentWave::Wave3 => CurrentWave::Wave1,
        };
    }
}
