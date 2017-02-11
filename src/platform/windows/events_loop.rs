
use std::os::windows::AsRawHandle;
use std::thread::Thread;
use Event;

pub struct EventsLoop {
    closures_queue: Arc<Mutex<Vec<Box<FnMut() + Send>>>>,
    current_msg_handler: Arc<Mutex<>>,
    thread: Thread,
}

impl EventsLoop {
    pub fn new() -> EventsLoop {
        let queue = Arc::new(Mutex::new(Vec::with_capacity(12)));

        {
            let queue = queue.clone();
            thread::spawn(move || {
                unsafe {
                    loop {
                        let mut msg = mem::uninitialized();

                        if user32::GetMessageW(&mut msg, ptr::null_mut(), 0, 0) == 0 {
                            break;
                        }

                        if msg.message == winapi::WM_USER {
                            let mut queue = closures_queue.lock().unwrap();
                            while !queue.is_empty() {
                                let f = queue.remove(0);
                                f();
                            }
                        }

                        user32::TranslateMessage(&msg);
                        user32::DispatchMessageW(&msg);   // calls `callback` (see the callback module)
                    }
                }
            });
        }
    }

    /// Executes some code in the background thread of the events loop.
    ///
    /// This method is specific to the win32 implementation and not public.
    pub fn exec_closure_in_thread<F>(&self)
        where F: FnMut()
    {
        unsafe {
            // TODO: error checking
            user32::PostThreadMessageW(self.thread.as_raw_handle(), winapi::WM_USER, 0, 0);
        }
    }

    pub fn interrupt(&self) {
    }

    pub fn poll_events<F>(&self, mut callback: F)
        where F: FnMut(Event)
    {
        
    }

    pub fn run_forever<F>(&self, mut callback: F)
        where F: FnMut(Event)
    {
        
    }
}
