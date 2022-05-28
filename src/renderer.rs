use crate::simulation::Simulation;
use colosseum::{Input, Vertex, Window};

pub struct Renderer {
    mesh: alexandria::Mesh<Vertex>,
}

impl Renderer {
    pub fn new<I: Input>(simulation: &Simulation, window: &mut Window<I>) -> Self {
        let mut vertices =
            Vec::with_capacity(simulation.num_points_x() * simulation.num_points_y());
        let mut indices = Vec::with_capacity(
            (simulation.num_points_x() - 1) * (simulation.num_points_y() - 1) * 6,
        );

        let base_x = -(simulation.width() / 2.0);
        let base_y = -(simulation.height() / 2.0);
        for y in 0..simulation.num_points_y() {
            for x in 0..simulation.num_points_x() {
                vertices.push(Vertex::new(
                    base_x + (x as f32) * simulation.dx(),
                    0.0,
                    base_y + (y as f32) * simulation.dy(),
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                    0.0,
                    0.0,
                ));

                if x != simulation.num_points_x() - 1 && y != simulation.num_points_y() - 1 {
                    let zero = (x + y * simulation.num_points_x()) as u32;
                    let one = (x + (y + 1) * simulation.num_points_x()) as u32;
                    let two = (x + 1 + (y + 1) * simulation.num_points_x()) as u32;
                    let three = (x + 1 + y * simulation.num_points_x()) as u32;

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

        Renderer { mesh }
    }

    pub fn update<I: Input>(&mut self, simulation: &mut Simulation, window: &mut Window<I>) {
        alexandria::compute::copy_compute_to_vertex(
            simulation.current_wave(),
            &mut self.mesh,
            window.inner(),
        );
    }

    pub fn render<I: Input>(&mut self, window: &mut Window<I>) {
        self.mesh.render(window.inner());
    }
}
