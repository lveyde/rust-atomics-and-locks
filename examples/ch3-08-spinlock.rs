use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::thread;

static mut DATA: String = String::new();
static LOCKED: AtomicBool = AtomicBool::new(false);

fn f() {
    let thread_id = thread::current().id();
    println!("{:?} Started.", &thread_id);
    while LOCKED
        .compare_exchange(false, true, Acquire, Relaxed)
        .is_err()
    {
        println!("{:?} Acquiring lock...", &thread_id);
    }
    println!("{:?} Acquired lock.", &thread_id);
    // Safety: We hold the exclusive lock, so nothing else is accessing DATA.
    println!("{:?} Updating DATA...", &thread_id);
    unsafe { DATA.push('!') };
    println!("{:?} Updated DATA.", &thread_id);
    LOCKED.store(false, Release);
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
    assert!(unsafe { DATA.len() } == 100);
    assert!(unsafe { DATA.chars().all(|c| c == '!') });
}
