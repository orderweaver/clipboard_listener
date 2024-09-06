# Clipboard Listener

A cross-platform Rust crate for listening to clipboard events.

### Windows

The listening is event-driven.

Using the Win API, a hidden window is created and registered to receive clipboard update events (`WM_CLIPBOARDUPDATE`).

### macOS

The listening is polling-based.

Using the Cocoa framework, the general pasteboard (`NSPasteboard`) change count is monitored as an indirect indication of clipboard updates. 

### Linux

The listening is polling-based.

Using the x11_clipboard crate, the clipboard contents are retrieved and hashed using the twox_hash crate, and compared for changes as an indirect indication of clipboard updates.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
clipboard_listener = "0.1.0"
```
# Usage

First, import the clipboard_listener crate in your project.

Define a callback function that specifies what should happen when the clipboard content changes. 

This function can perform any custom behavior you need, such as logging the change or getting the clipboard content.

# Example

```rust
use clipboard_listener::listen_clipboard;

fn main() {
    // Prints a message indicating that the clipboard listener is starting
    println!("Starting clipboard listener...");

    // Defines the callback function to be called when the clipboard changes
    let callback = || {
        
        // Prints a message indicating that the clipboard content has updated
        println!("Clipboard updated!");

        // Add your own custom behavior here
        
    };

    // Starts the clipboard listener with the defined callback function
    if let Err(e) = clipboard_listener::listen_clipboard(Box::new(callback)) {
        // Prints an error message if the listener fails to start
        eprintln!("Error: {}", e);
    }
}
```
# Note

For macOS and linux, the polling interval can be adjusted by modifying the sleep duration in macos.rs and linux.rs.

It is recommended to not poll too frequently (<1 second) to reduce CPU usage.