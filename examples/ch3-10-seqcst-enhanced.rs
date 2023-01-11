use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::thread;

static A: AtomicBool = AtomicBool::new(false);
static B: AtomicBool = AtomicBool::new(false);

static mut S: String = String::new();

fn main() {
    let a = thread::spawn(|| {
        println!("Thread A");
        A.store(true, SeqCst);
        if !B.load(SeqCst) {
            unsafe { S.push('A') };
        } else {
            unsafe { S.push('a') };
        }
    });

    let b = thread::spawn(|| {
        println!("Thread B");
        B.store(true, SeqCst);
        if !A.load(SeqCst) {
            unsafe { S.push('B') };
        } else {
            unsafe { S.push('b') };
        }
    });

    a.join().unwrap();
    b.join().unwrap();

    unsafe { println!("{}", &S) };
}
