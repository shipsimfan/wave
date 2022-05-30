use colosseum::App;
use observer::Observer;
use renderer::Renderer;
use simulation::Simulation;

mod observer;
mod renderer;
mod simulation;

const DX: f32 = 1.0 / (NUM_POINTS_X as f32 - 1.0);
const DY: f32 = DX;

const NUM_POINTS_X: usize = 256;
const NUM_POINTS_Y: usize = NUM_POINTS_X;

const FRAME_TIME: f32 = 1.0 / 60.0;
const SUB_STEPS: f32 = 1.0;
const DT: f32 = FRAME_TIME / SUB_STEPS;

const C: f32 = 0.05;

pub struct Game {
    observer: Observer,
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
        let simulation = Simulation::new(NUM_POINTS_X, DX, NUM_POINTS_Y, DY, DT, C, window);
        let renderer = Renderer::new(&simulation, window);
        let observer = Observer::new(window);

        Game {
            observer,
            simulation,
            renderer,
            tick_time: 0.0,
        }
    }

    fn update(&mut self, delta_time: f32, window: &mut colosseum::Window<Self::Input>) {
        // Camera update
        self.observer.update(delta_time, window);

        // Physics update
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
        self.observer.set_active(window);

        self.renderer.render(window);
    }

    fn clear_color(&self) -> [f32; 4] {
        [0.0, 0.0, 0.0, 1.0]
    }
}
