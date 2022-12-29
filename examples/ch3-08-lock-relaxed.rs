use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

static mut DATA: String = String::new();
static LOCKED: AtomicBool = AtomicBool::new(false);

fn f() {
    if LOCKED
        .compare_exchange(false, true, Relaxed, Relaxed)
        .is_ok()
    {
        // Safety: We hold the exclusive lock, so nothing else is accessing DATA.
        unsafe { DATA.push('!') };
        LOCKED.store(false, Relaxed);
    } else {
        println!(
            "{:?} Error - LOCKED is already set to true.",
            thread::current().id()
        );
    }
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
