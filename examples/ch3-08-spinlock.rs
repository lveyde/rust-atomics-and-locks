use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::thread;

static mut DATA: String = String::new();
static LOCKED: AtomicBool = AtomicBool::new(false);

fn f() {
    if LOCKED.compare_exchange(false, true, Acquire, Relaxed).is_err() {
        println!("{:?} Acquiring lock...", thread::current().id());
    }
    // Safety: We hold the exclusive lock, so nothing else is accessing DATA.
    println!("{:?} Updating...", thread::current().id());
    unsafe { DATA.push('!') };
    LOCKED.store(false, Release);
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
