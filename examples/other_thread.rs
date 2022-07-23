// Windows only currently
use std::os::windows::prelude::AsRawHandle;
use backtrace::{Backtrace, BacktraceFrame};


fn worker() {
    foo();
}
fn foo() {
    bar()
}
fn bar() {
    baz()
}
fn baz() {
    //println!("Hello from thread!");
    // Sleep for simple sync. Can't read thread that has finished running
    //std::thread::sleep(std::time::Duration::from_millis(1000));
    loop {
        print!("");
    }
}

fn main() {
    for index in 0..100 {
        //std::thread::sleep_ms(1000);
        let thread = std::thread::spawn(|| {
            worker();
        });
        let os_handle = thread.as_raw_handle();

        // Allow the thread to start
        std::thread::sleep(std::time::Duration::from_millis(500));

        let mut frames = Vec::new();
        unsafe {
            backtrace::trace_thread_unsynchronized(os_handle, |frame| {
                frames.push(BacktraceFrame::from(frame.clone()));
                true
            });
        }
        let len = frames.len();
        let mut bt = Backtrace::from(frames);
        bt.resolve();

        if let Some(symbol) = bt.frames().last().unwrap().symbols()[0].name() {
            let name = symbol.to_string();
            if name != "RtlUserThreadStart" || index == 0 {
                println!("{index}:{}, {:?}", len, bt);
            }
        } else {
            panic!("no good");
        }
    }
}
