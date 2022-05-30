use crate::simulation_runner::SimulationRunner;
use colosseum::{Input, Vertex, Window};

pub struct Renderer {
    mesh: alexandria::Mesh<Vertex>,
    shader: colosseum::Shader,
    texture: alexandria::Texture,
}

impl Renderer {
    pub fn new<I: Input>(
        simulation: &SimulationRunner,
        num_points_x: usize,
        num_points_y: usize,
        window: &mut Window<I>,
    ) -> Self {
        let mut vertices = Vec::with_capacity(num_points_x * num_points_y);
        let mut indices = Vec::with_capacity((num_points_x - 1) * (num_points_y - 1) * 6);

        let base_x = -(simulation.width() / 2.0);
        let base_y = -(simulation.height() / 2.0);
        for y in 0..num_points_y {
            for x in 0..num_points_x {
                vertices.push(Vertex::new(
                    base_x + (x as f32) * simulation.dx(),
                    0.0,
                    base_y + (y as f32) * simulation.dy(),
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    x as f32 / num_points_x as f32,
                    y as f32 / num_points_y as f32,
                ));

                if x != num_points_x - 1 && y != num_points_y - 1 {
                    let zero = (x + y * num_points_x) as u32;
                    let one = (x + (y + 1) * num_points_x) as u32;
                    let two = (x + 1 + (y + 1) * num_points_x) as u32;
                    let three = (x + 1 + y * num_points_x) as u32;

                    indices.push(zero);
                    indices.push(one);
                    indices.push(two);
                    indices.push(two);
                    indices.push(three);
                    indices.push(zero);
                }
            }
        }

        let mesh =
            alexandria::Mesh::new(vertices.as_slice(), indices.as_slice(), window.inner()).unwrap();

        let shader = colosseum::Shader::new(include_str!("shader.hlsl"), window);

        let initial_values = vec![0.0; simulation.num_points_x() * simulation.num_points_y()];
        let texture = alexandria::Texture::new_1f(
            initial_values.as_slice(),
            simulation.num_points_x(),
            0,
            window.inner(),
        );

        Renderer {
            mesh,
            shader,
            texture,
        }
    }

    pub fn update<I: Input>(&mut self, simulation: &mut SimulationRunner, window: &mut Window<I>) {
        window.inner().device_context().flush();
        window
            .inner()
            .device_context()
            .copy_resource(self.texture.inner_mut(), simulation.output().inner_mut())
    }

    pub fn render<I: Input>(&mut self, window: &mut Window<I>) {
        self.shader.set_active(window);
        self.texture.set_active(window.inner());
        self.mesh.render(window.inner());
    }
}
