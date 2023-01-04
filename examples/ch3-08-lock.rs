use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::thread;

static mut DATA: String = String::new();
static LOCKED: AtomicBool = AtomicBool::new(false);

fn f() {
    let thread_id = thread::current().id();
    println!("{:?} Started.", &thread_id);
    if LOCKED
        .compare_exchange(false, true, Acquire, Relaxed)
        .is_ok()
    {
        // Safety: We hold the exclusive lock, so nothing else is accessing DATA.
        unsafe { DATA.push('!') };
        println!("{:?} Updated DATA.", &thread_id);
        LOCKED.store(false, Release);
    } else {
        println!("{:?} Error - LOCKED is already set to true.", &thread_id);
    }
    println!("{:?} Finished.", &thread_id);
}

fn main() {
    thread::scope(|s| {
        for _ in 0..100 {
            s.spawn(f);
        }
    });
    println!("{}, {}", unsafe { &DATA }, unsafe { (&DATA).len() });
    // DATA now contains at least one exclamation mark (and maybe more).
    assert!(unsafe { DATA.len() } > 0);
    assert!(unsafe { DATA.chars().all(|c| c == '!') });
}
