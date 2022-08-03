use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::sleep,
    time::Duration,
};

use super::state::State;

pub struct UI {
    pub state: State,
    should_render: Arc<AtomicBool>,
}

impl UI {
    pub fn new() -> Self {
        Self {
            state: State::new(),
            should_render: Arc::new(AtomicBool::new(true)),
        }
    }

    pub fn setup(&self) {}

    pub fn cleanup(&self) {}

    pub fn render(&self) {}
}

pub fn start_rendering(ui: Arc<UI>) {
    let should_render = ui.should_render.clone();

    tokio::spawn(async move {
        loop {
            let result =
                should_render.compare_exchange(true, false, Ordering::Relaxed, Ordering::Relaxed);

            if result.is_ok() {
                ui.render();
            }

            // TODO: Find a better way to reactively update
            sleep(Duration::from_millis(50));
        }
    });
}
