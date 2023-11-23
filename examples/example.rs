use swil::{WindowConfig, Window, event::Event};

fn main() {
    let config = WindowConfig::new();
    let window = Window::new(&config).unwrap();

    window.event_loop(|event, exit| {
        println!("{event:?}");

        if let Event::CloseRequested = event {
            *exit = true;
        }
    }).unwrap();
}