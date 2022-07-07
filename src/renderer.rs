use crate::{simulation_runner::SimulationRunner, Simulation};
use colosseum::{Input, Vertex, Window};

pub struct Renderer {
    mesh: colosseum::MeshRenderer,
    shader: colosseum::Shader,
    texture: alexandria::Texture,
}

impl Renderer {
    pub fn new<I: Input, S: Simulation>(
        simulation_runner: &SimulationRunner,
        simulation: &S,
        window: &mut Window<I>,
    ) -> Self {
        let settings = simulation.render_settings();

        let mut vertices = Vec::with_capacity(settings.num_points_x() * settings.num_points_y());
        let mut indices =
            Vec::with_capacity((settings.num_points_x() - 1) * (settings.num_points_y() - 1) * 6);

        let base_x = -(simulation_runner.width() / 2.0);
        let base_y = -(simulation_runner.height() / 2.0);
        for y in 0..settings.num_points_y() {
            for x in 0..settings.num_points_x() {
                vertices.push(Vertex::new(
                    base_x + (x as f32) * simulation_runner.dx(),
                    0.0,
                    base_y + (y as f32) * simulation_runner.dy(),
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    x as f32 / settings.num_points_x() as f32,
                    y as f32 / settings.num_points_y() as f32,
                ));

                if x != settings.num_points_x() - 1 && y != settings.num_points_y() - 1 {
                    let zero = (x + y * settings.num_points_x()) as u32;
                    let one = (x + (y + 1) * settings.num_points_x()) as u32;
                    let two = (x + 1 + (y + 1) * settings.num_points_x()) as u32;
                    let three = (x + 1 + y * settings.num_points_x()) as u32;

                    indices.push(zero);
                    indices.push(one);
                    indices.push(two);
                    indices.push(two);
                    indices.push(three);
                    indices.push(zero);
                }
            }
        }

        let mut mesh = colosseum::MeshRenderer::new(colosseum::Mesh::new(
            vertices.as_slice(),
            indices.as_slice(),
            window,
        ));

        mesh.transform_mut().set_scale(colosseum::Vector3::new(
            settings.xz_scale(),
            settings.y_scale(),
            settings.xz_scale(),
        ));

        let shader = colosseum::Shader::new(include_str!("shader.hlsl"), window);

        let initial_values =
            vec![0.0; simulation_runner.num_points_x() * simulation_runner.num_points_y()];
        let texture = alexandria::Texture::new_1f(
            initial_values.as_slice(),
            simulation_runner.num_points_x(),
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
        self.mesh.render(window);
    }
}
