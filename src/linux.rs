extern crate x11_clipboard;
extern crate twox_hash;

use x11_clipboard::Clipboard;
use twox_hash::XxHash32;
use std::error::Error;
use std::time::Duration;
use std::thread::sleep;
use std::hash::Hasher;

pub type ClipboardCallback = Box<dyn Fn() + Send + Sync>;

fn hash_content(content: &[u8]) -> u32 {
    let mut hasher = XxHash32::default();
    hasher.write(content);
    hasher.finish() as u32
}

pub fn listen_clipboard(callback: ClipboardCallback) -> Result<(), Box<dyn Error>> {
    let clipboard = Clipboard::new()?;
    
    // Initialize with the first clipboard content
    let mut last_content = clipboard.load(
        clipboard.getter.atoms.clipboard,
        clipboard.getter.atoms.utf8_string,
        clipboard.getter.atoms.property,
        Duration::from_millis(1000), 
    )?;
    
    let mut last_hash = hash_content(&last_content);

    loop {
        let current_content = clipboard.load(
            clipboard.getter.atoms.clipboard,
            clipboard.getter.atoms.utf8_string,
            clipboard.getter.atoms.property,
            Duration::from_millis(1000), 
        )?;

        let current_hash = hash_content(&current_content);

        // Check if the content has changed
        if current_hash != last_hash {
            callback();
            last_hash = current_hash; // Update last hash
            last_content = current_content; // Update last content
        }

        sleep(Duration::from_millis(1000)); // 1000 milliseconds = 1 second
    }
}
