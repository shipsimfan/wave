use std::f32::consts::PI;

pub struct Simulation {
    wave1: Box<[f32]>,
    wave2: Box<[f32]>,
    current_wave: bool,
    width: f32,
    dx: f32,
    c: f32,
}

impl Simulation {
    pub fn new(width: f32, dx: f32, wave_speed: f32) -> Self {
        let num_points = (width / dx) as usize + 1;

        let mut wave = Vec::with_capacity(num_points);
        for i in 0..num_points {
            let x = i as f32 * dx;

            if x < 0.35 || x > 0.65 {
                wave.push(0.0);
            } else {
                wave.push((1.0 / (0.1) * PI * width * (x - 0.35)).sin() * 0.2);
            }
        }

        let wave1 = wave.into_boxed_slice();
        let wave2 = wave1.clone();

        Simulation {
            wave1,
            wave2,
            current_wave: true,
            width,
            dx,
            c: wave_speed,
        }
    }

    pub fn update(&mut self, dt: f32) {
        let r = (self.c * dt / self.dx) * (self.c * dt / self.dx);

        for i in 0..self.wave1.len() {
            let (u_now, u_last, lu, uu) = self.get_values(i);

            let new_u = 2.0 * u_now - u_last + r * (uu - 2.0 * u_now + lu);

            self.update_value(i, new_u);
        }

        self.current_wave = !self.current_wave;
    }

    pub fn wave(&self) -> &[f32] {
        if self.current_wave {
            &self.wave1
        } else {
            &self.wave2
        }
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn dx(&self) -> f32 {
        self.dx
    }

    fn last_wave(&self) -> &[f32] {
        if self.current_wave {
            &self.wave2
        } else {
            &self.wave1
        }
    }

    fn get_values(&self, x: usize) -> (f32, f32, f32, f32) {
        let current_wave = self.wave();
        let last_wave = self.last_wave();

        (
            current_wave[x],
            last_wave[x],
            if x == 0 { 0.0 } else { current_wave[x - 1] },
            if x == current_wave.len() - 1 {
                0.0
            } else {
                current_wave[x + 1]
            },
        )
    }

    fn update_value(&mut self, x: usize, u: f32) {
        if self.current_wave {
            self.wave2[x] = u;
        } else {
            self.wave1[x] = u;
        }
    }
}
