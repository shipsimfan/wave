use crate::simulation::Simulation;
use colosseum::{Input, Vertex, Window};

pub struct Renderer {
    mesh: alexandria::LineMesh<Vertex>,
}

impl Renderer {
    pub fn new<I: Input>(simulation: &Simulation, window: &mut Window<I>) -> Self {
        let mut vertices = Vec::with_capacity(simulation.num_points());

        let base = -0.5;
        for i in 0..simulation.num_points() {
            vertices.push(Vertex::new(
                base + (i as f32) * 0.001,
                0.0,
                0.0,
                1.0,
                1.0,
                1.0,
                1.0,
                0.0,
                0.0,
            ));
        }

        let mesh = alexandria::LineMesh::new(vertices.as_slice(), true, window.inner()).unwrap();

        Renderer { mesh }
    }

    pub fn update<I: Input>(&mut self, simulation: &mut Simulation, window: &mut Window<I>) {
        alexandria::compute::copy_compute_to_vertex_line(
            simulation.current_wave(),
            &mut self.mesh,
            window.inner(),
        );
    }

    pub fn render<I: Input>(&mut self, window: &mut Window<I>) {
        self.mesh.render(window.inner());
    }
}
