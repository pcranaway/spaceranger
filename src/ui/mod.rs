use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::sleep,
    time::Duration,
};

use self::ui::UI;

pub mod state;
pub mod ui;

pub fn init_renderer() -> Arc<AtomicBool> {
    let ui = UI::new();

    let should_render: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
    let should_render_clone = should_render.clone();

    tokio::spawn(async move {
        loop {
            let result = should_render_clone.compare_exchange(
                true,
                false,
                Ordering::Relaxed,
                Ordering::Relaxed,
            );

            if result.is_ok() {
                ui.render();
            }

            // TODO: Find a better way to reactively update
            sleep(Duration::from_millis(50));
        }
    });

    should_render.clone()
}
