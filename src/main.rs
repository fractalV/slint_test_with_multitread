use std::thread;
use std::time::Duration;

slint::include_modules!();

/// Main function that initializes a counter and spawns two threads to update it.
/// 
/// The first thread increments the counter by 1 every second.
/// The second thread increments the counter by 10 every two seconds.
/// The counter's display text is updated accordingly in both threads.
/// 
/// # Panics
/// 
/// This function will panic if the counter fails to initialize or if the event loop fails to run.
fn main() {
    let counter = MyCounter::new().unwrap();
    
    let counter_weak1 = counter.as_weak();
    let counter_weak2 = counter.as_weak();    
    
    counter.set_count(0);
    counter.set_display_text("Count: 0".into());

    thread::spawn(move || {        
        loop {
            thread::sleep(Duration::from_secs(1));            
            counter_weak1.upgrade_in_event_loop(move |counter| {
                let mut c = counter.get_count();
                c += 1;
                counter.set_count(c);
                counter.set_display_text(format!("Count: {}", c).into());
            }).unwrap();
        }
    });

    thread::spawn(move || {        
         loop {
            thread::sleep(Duration::from_secs(2));
            counter_weak2.upgrade_in_event_loop(move |counter| {
                let mut c = counter.get_count();
                c += 10;
                counter.set_count(c);
                counter.set_display_text(format!("Count: {}", c).into());
            }).unwrap();
         }
    });
        
    counter.run().unwrap();
}