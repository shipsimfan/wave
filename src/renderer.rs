use crate::simulation::Simulation;
use colosseum::{Input, Mesh, MeshRenderer, Vertex, Window};

pub struct Renderer {
    vertices: Box<[Vertex]>,
    mesh_renderer: MeshRenderer,
}

impl Renderer {
    pub fn new<I: Input>(simulation: &Simulation, window: &mut Window<I>) -> Self {
        let mut vertices = Vec::with_capacity(simulation.wave().len());

        let base = -(simulation.width() / 2.0);
        for i in 0..simulation.wave().len() {
            vertices.push(Vertex::new(
                base + i as f32 * simulation.dx(),
                simulation.wave()[i],
                0.0,
                1.0,
                1.0,
                1.0,
                1.0,
                0.0,
                0.0,
            ));
        }

        let mesh_renderer = MeshRenderer::new(Mesh::new_line(&vertices, true, window));

        Renderer {
            vertices: vertices.into_boxed_slice(),
            mesh_renderer,
        }
    }

    pub fn update<I: Input>(&mut self, simulation: &Simulation, window: &mut Window<I>) {
        for i in 0..simulation.wave().len() {
            self.vertices[i].position_mut().set_y(simulation.wave()[i]);
        }

        self.mesh_renderer
            .set_mesh(Mesh::new_line(&self.vertices, true, window));
    }

    pub fn render<I: Input>(&mut self, window: &mut Window<I>) {
        self.mesh_renderer.render(window);
    }
}
