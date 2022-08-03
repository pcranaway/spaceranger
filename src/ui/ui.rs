use std::{
    error::Error,
    io::stdout,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::sleep,
    time::Duration,
};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, size, ScrollUp, SetSize},
};
use tokio::sync::Mutex;

use super::state::State;

pub struct UI {
    pub state: State,
    should_render: AtomicBool,
    size: (u16, u16),
}

impl UI {
    pub fn new() -> Self {
        Self {
            state: State::new(),
            should_render: AtomicBool::new(true),
            size: (1, 10),
        }
    }

    pub fn setup(&mut self) -> Result<(), Box<dyn Error>> {
        // get the terminal size (TODO: be update if resized)
        self.size = size()?;

        enable_raw_mode()?;

        Ok(())
    }

    pub fn cleanup(&self) -> Result<(), Box<dyn Error>> {
        disable_raw_mode()?;

        Ok(())
    }

    pub fn render(&self) {}
}

pub fn start_rendering(ui: Arc<Mutex<UI>>) {
    tokio::spawn(async move {
        loop {
            let lock = ui.lock().await;

            let result = lock.should_render.compare_exchange(
                true,
                false,
                Ordering::Relaxed,
                Ordering::Relaxed,
            );

            if result.is_ok() {
                lock.render();
            }

            // TODO: Find a better way to reactively update
            sleep(Duration::from_millis(50));
        }
    });
}
