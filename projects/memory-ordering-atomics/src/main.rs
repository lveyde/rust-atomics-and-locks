//! Copyright (C) 2023 - Lev Veyde <lveyde@gmail.com>
//! Inspired by https://preshing.com/20120515/memory-reordering-caught-in-the-act/

use rsevents_extra::Semaphore;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::SeqCst;
use std::thread;

static THREAD1: Semaphore = Semaphore::new(0, 1);
static THREAD2: Semaphore = Semaphore::new(0, 1);
static END: Semaphore = Semaphore::new(0, 2);

static mut A: AtomicU64 = AtomicU64::new(0);
static mut B: AtomicU64 = AtomicU64::new(0);
static mut X: AtomicU64 = AtomicU64::new(0);
static mut Y: AtomicU64 = AtomicU64::new(0);

fn thread1() {
    loop {
        THREAD1.wait().forget();

        unsafe {
            A.store(1, SeqCst);
            Y.store(B.load(SeqCst), SeqCst);
        }

        END.release(1);
    }
}

fn thread2() {
    loop {
        THREAD2.wait().forget();

        unsafe {
            B.store(1, SeqCst);
            X.store(A.load(SeqCst), SeqCst);
        }

        END.release(1);
    }
}

fn main() {
    let mut iterations = 0;

    thread::spawn(thread1);
    thread::spawn(thread2);

    loop {
        unsafe {
            A.store(0, SeqCst);
            B.store(0, SeqCst);
        }

        THREAD1.release(1);
        THREAD2.release(1);

        END.wait().forget();
        END.wait().forget();

        iterations += 1;

        if iterations % 1_000_000 == 0 {
            println!("Passed {} iterations", iterations);
        }

        unsafe {
            if X.load(SeqCst) == 0 && Y.load(SeqCst) == 0 {
                println!("Iteration: {}, X and Y == 0", iterations);
            }
        }
    }
}
