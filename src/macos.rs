extern crate cocoa;
extern crate objc;

use cocoa::appkit::{NSApplication, NSApp, NSPasteboard};
use cocoa::base::nil;
use cocoa::foundation::NSAutoreleasePool;
use std::error::Error;
use std::time::Duration;
use crate::ClipboardCallback;

pub fn listen_clipboard(callback: ClipboardCallback) -> Result<(), Box<dyn Error>> {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);
        let app = NSApp();
        app.setActivationPolicy_(cocoa::appkit::NSApplicationActivationPolicyProhibited);

        let pasteboard = NSPasteboard::generalPasteboard(nil);
        let mut count = pasteboard.changeCount();

        loop {
            let new_count = pasteboard.changeCount();
            if new_count != count {
                callback();
                count = new_count;
            }
            std::thread::sleep(Duration::from_millis(1000)); // 1000 milliseconds = 1 second
        }
    }
}
