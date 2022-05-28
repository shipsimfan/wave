use colosseum::{Input, Vertex, Window};
use std::{f32::consts::PI, ffi::CString};

enum CurrentWave {
    Wave1,
    Wave2,
    Wave3,
}

pub struct Simulation {
    num_points: usize,
    current_wave: CurrentWave,

    // Wave buffers
    wave_buffer1: win32::ID3D11Buffer,
    wave_buffer2: win32::ID3D11Buffer,
    wave_buffer3: win32::ID3D11Buffer,

    // Views
    wave_view1: win32::ID3D11UnorderedAccessView,
    wave_view2: win32::ID3D11UnorderedAccessView,
    wave_view3: win32::ID3D11UnorderedAccessView,

    // Compute shader
    compute_shader: win32::ID3D11ComputeShader,
}

impl Simulation {
    pub fn new<I: Input>(width: f32, dx: f32, window: &mut Window<I>) -> Self {
        let num_points = (width / dx).round() as usize + 1;

        let compute_shader_code = CString::new(include_str!("compute.hlsl")).unwrap();
        let window = window.inner();
        let device = window.device();

        let (shader_blob, errors) = win32::d3d_compile(
            &compute_shader_code,
            None,
            &[],
            Some(&CString::new("compute_main").unwrap()),
            &CString::new("cs_5_0").unwrap(),
            &[win32::D3DCompileFlag::EnableStrictness],
            &[],
        );

        match errors {
            Some(mut error) => println!("{}", error.as_str().unwrap()),
            None => {}
        }

        let shader_blob = shader_blob.unwrap();
        let compute_shader = device.create_compute_shader(&shader_blob).unwrap();

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

        let buffer_desc = win32::D3D11BufferDesc::new(
            (std::mem::size_of::<Vertex>() * vertices.len()) as u32,
            win32::D3D11Usage::Default,
            &[win32::D3D11BindFlag::UnorderedAccess],
            &[],
            &[win32::D3D11ResourceMiscFlag::BufferStructured],
            std::mem::size_of::<Vertex>() as u32,
        );
        let initial_data = win32::D3D11SubresourceData::new(vertices.as_slice(), 0, 0);
        let mut wave_buffer1 = device
            .create_buffer(&buffer_desc, Some(&initial_data))
            .unwrap();
        let mut wave_buffer2 = device
            .create_buffer(&buffer_desc, Some(&initial_data))
            .unwrap();
        let mut wave_buffer3 = device
            .create_buffer(&buffer_desc, Some(&initial_data))
            .unwrap();

        let uav_desc =
            win32::D3D11UnorderedAccessViewDesc::new(win32::DXGIFormat::Unknown, &mut wave_buffer1);
        let wave_view1 = device
            .create_unordered_access_view(&mut wave_buffer1, &uav_desc)
            .unwrap();
        let wave_view2 = device
            .create_unordered_access_view(&mut wave_buffer2, &uav_desc)
            .unwrap();
        let wave_view3 = device
            .create_unordered_access_view(&mut wave_buffer3, &uav_desc)
            .unwrap();

        Simulation {
            num_points,
            current_wave: CurrentWave::Wave1,
            compute_shader,
            wave_buffer1,
            wave_buffer2,
            wave_buffer3,
            wave_view1,
            wave_view2,
            wave_view3,
        }
    }

    pub fn num_points(&self) -> usize {
        self.num_points
    }

    pub fn update<I: Input>(&mut self, window: &mut Window<I>) {
        let dc = window.inner().device_context();
        dc.cs_set_shader(&mut self.compute_shader);
        dc.cs_set_unordered_access_views(0, &mut self.wave_views());
        dc.dispatch(self.num_points as u32, 1, 1);

        self.set_next_wave()
    }

    pub fn wave_buffer(&mut self) -> &mut win32::ID3D11Buffer {
        match self.current_wave {
            CurrentWave::Wave1 => &mut self.wave_buffer1,
            CurrentWave::Wave2 => &mut self.wave_buffer2,
            CurrentWave::Wave3 => &mut self.wave_buffer3,
        }
    }

    fn wave_views(&mut self) -> [&mut win32::ID3D11UnorderedAccessView; 3] {
        match self.current_wave {
            CurrentWave::Wave1 => [
                &mut self.wave_view3,
                &mut self.wave_view1,
                &mut self.wave_view2,
            ],
            CurrentWave::Wave2 => [
                &mut self.wave_view1,
                &mut self.wave_view2,
                &mut self.wave_view3,
            ],
            CurrentWave::Wave3 => [
                &mut self.wave_view2,
                &mut self.wave_view3,
                &mut self.wave_view1,
            ],
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
