# shuten_core

Shuten is an opionate crate for drawing terminal interfaces

This is the low level `core` crate that provides simple drawing and event handling

### Basic usage:

```rust
use shuten_core::{Terminal, event::Event, Cell};
// create a new terminal
let mut terminal = Terminal::new(Config::default())?;
// wait for an event
while let Ok(event) = terminal.wait_for_next_event() {
    // if its a quit event, just break
    if event.is_quit() { break }

    // get a canvas to paint on
    terminal.paint(|mut canvas| {
        // fill it with `red`
        canvas.fill(0xFF0000);
    })?;
}

```

LICENSE: 0-BSD
