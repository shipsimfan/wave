use colosseum::{App, Camera, Projection, Vector3};
use renderer::Renderer;
use simulation::Simulation;

mod renderer;
mod simulation;

const SIMULATION_WIDTH: f32 = 1.0;
const DX: f32 = 0.001;
const DT: f32 = 1.0 / 60.0;
const C: f32 = 0.05;

struct Game {
    camera: Camera,
    simulation: Simulation,
    renderer: Renderer,
    tick_time: f32,
}
fn main() {
    App::<Game>::new();
}

impl colosseum::Game for Game {
    const INITIAL_TITLE: &'static str = "Wave Simulator";

    fn new(window: &mut colosseum::Window<Self::Input>) -> Self {
        let mut camera = Camera::new(window);
        camera.set_projection(
            Projection::orthographic(SIMULATION_WIDTH, 0.01, 10.0),
            window,
        );
        camera
            .transform_mut()
            .set_position(Vector3::new(0.0, 0.0, 1.0));

        let simulation = Simulation::new(SIMULATION_WIDTH, DX, window);
        let renderer = Renderer::new(&simulation, window);

        Game {
            camera,
            simulation,
            renderer,
            tick_time: 0.0,
        }
    }

    fn update(&mut self, delta_time: f32, window: &mut colosseum::Window<Self::Input>) {
        self.tick_time += delta_time;

        if self.tick_time >= DT {
            while self.tick_time >= DT {
                self.simulation.update(window);
                self.tick_time -= DT;
            }

            self.renderer.update(&mut self.simulation, window);
        }
    }

    fn render(&mut self, window: &mut colosseum::Window<Self::Input>) {
        self.camera.set_active(window);

        self.renderer.render(window);
    }

    fn clear_color(&self) -> [f32; 4] {
        [0.0, 0.0, 0.0, 1.0]
    }
}
