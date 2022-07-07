use observer::Observer;
use renderer::Renderer;
use simulation_runner::SimulationRunner;
use std::marker::PhantomData;

mod observer;
mod renderer;
mod simulation;
mod simulation_runner;

pub use simulation::{RenderSettings, Simulation, SimulationSettings};

struct Game<S: Simulation> {
    observer: Observer,
    simulation_runner: SimulationRunner,
    renderer: Renderer,
    tick_time: f32,
    time_scale: f32,
    phantom: PhantomData<S>,
}

pub fn run<S: Simulation>() -> ! {
    colosseum::App::<Game<S>>::new();
}

impl<S: Simulation> colosseum::Game for Game<S> {
    const INITIAL_TITLE: &'static str = "Wave Simulator";

    fn new(window: &mut colosseum::Window<Self::Input>) -> Self {
        let simulation = S::new();

        let simulation_runner = SimulationRunner::new(&simulation, window);
        let renderer = Renderer::new(&simulation_runner, &simulation, window);
        let observer = Observer::new(window);

        Game {
            observer,
            simulation_runner,
            renderer,
            tick_time: 0.0,
            time_scale: simulation.time_scale(),
            phantom: PhantomData,
        }
    }

    fn update(&mut self, delta_time: f32, window: &mut colosseum::Window<Self::Input>) {
        // Camera update
        self.observer.update(delta_time, window);

        // Physics update
        self.tick_time += delta_time / self.time_scale;
        if self.tick_time >= self.simulation_runner.dt() {
            while self.tick_time >= self.simulation_runner.dt() {
                self.simulation_runner.update(window);
                self.tick_time -= self.simulation_runner.dt();
            }

            self.renderer.update(&mut self.simulation_runner, window);
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
