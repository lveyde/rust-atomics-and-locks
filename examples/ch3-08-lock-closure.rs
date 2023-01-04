use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::sync::Arc;
use std::thread;

static mut DATA: String = String::new();

fn main() {
    let locked: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));

    thread::scope(|s| {
        for _ in 0..100 {
            s.spawn(|| {
                let thread_id = thread::current().id();
                println!("{:?} Started.", &thread_id);
                while locked.compare_exchange(0, 1, Acquire, Relaxed).is_err() {
                    println!("{:?} Acquiring lock...", &thread_id);
                }
                println!("{:?} Acquired lock.", &thread_id);
                // Safety: We hold the exclusive lock, so nothing else is accessing DATA.
                println!("{:?} Updating DATA...", &thread_id);
                unsafe { DATA.push('!') };
                println!("{:?} Updated DATA.", &thread_id);
                locked.store(0, Release);
                println!("{:?} Finished.", &thread_id);
            });
        }
    });
    println!("{}, {}", unsafe { &DATA }, unsafe { (&DATA).len() });
    // DATA now contains at least one exclamation mark (and maybe more).
    assert!(unsafe { DATA.len() } > 0);
    assert!(unsafe { DATA.chars().all(|c| c == '!') });
}
