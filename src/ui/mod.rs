use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use self::ui::UI;

pub mod ui;

pub fn init_renderer() -> Arc<AtomicBool> {
    let ui = UI::new();

    let should_render: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let should_render_clone = should_render.clone();

    tokio::spawn(async move {
        loop {
            if true == should_render_clone.load(Ordering::Relaxed) {
                ui.render();
            }
        }
    });

    should_render.clone()
}
