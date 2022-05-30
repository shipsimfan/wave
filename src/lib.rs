use observer::Observer;
use renderer::Renderer;
use simulation_runner::SimulationRunner;
use std::marker::PhantomData;

mod observer;
mod renderer;
mod simulation;
mod simulation_runner;

pub use simulation::Simulation;

struct Game<S: Simulation> {
    observer: Observer,
    simulation: SimulationRunner,
    renderer: Renderer,
    tick_time: f32,
    phantom: PhantomData<S>,
}

pub fn run<S: Simulation>(simulation: S) {}

impl<S: Simulation> colosseum::Game for Game<S> {
    const INITIAL_TITLE: &'static str = "Wave Simulator";

    fn new(window: &mut colosseum::Window<Self::Input>) -> Self {
        let simulation = SimulationRunner::new(0, 0.0, 0, 0.0, 0.0, 0.0, window);
        let renderer = Renderer::new(&simulation, 0, 0, window);
        let observer = Observer::new(window);

        Game {
            observer,
            simulation,
            renderer,
            tick_time: 0.0,
            phantom: PhantomData,
        }
    }

    fn update(&mut self, delta_time: f32, window: &mut colosseum::Window<Self::Input>) {
        // Camera update
        self.observer.update(delta_time, window);

        // Physics update
        self.tick_time += delta_time;
        if self.tick_time >= 0.0 {
            while self.tick_time >= 0.0 {
                self.simulation.update(window);
                self.tick_time -= 0.0;
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
