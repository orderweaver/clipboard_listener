use clipboard_listener::listen_clipboard;

fn main() {
    println!("Starting clipboard listener...");
    
    let callback = || {
        println!("Clipboard updated!");
        // Add more custom behavior here if needed
    };

    if let Err(e) = listen_clipboard(Box::new(callback)) {
        eprintln!("Error: {}", e);
    }
}
