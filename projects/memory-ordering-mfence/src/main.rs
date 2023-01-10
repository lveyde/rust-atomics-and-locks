//! Copyright (C) 2023 - Lev Veyde <lveyde@gmail.com>
//! Inspired by https://preshing.com/20120515/memory-reordering-caught-in-the-act/

use rsevents_extra::Semaphore;
use std::arch::asm;
use std::thread;

static THREAD1: Semaphore = Semaphore::new(0, 1);
static THREAD2: Semaphore = Semaphore::new(0, 1);
static END: Semaphore = Semaphore::new(0, 2);

static mut A: u64 = 0;
static mut B: u64 = 0;
static mut X: u64 = 0;
static mut Y: u64 = 0;

fn thread1() {
    loop {
        THREAD1.wait().forget();

        unsafe {
            A = 1;
            asm!("mfence");
            Y = B;
        }

        END.release(1);
    }
}

fn thread2() {
    loop {
        THREAD2.wait().forget();

        unsafe {
            B = 1;
            asm!("mfence");
            X = A;
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
            A = 0;
            B = 0;
        }

        THREAD1.release(1);
        THREAD2.release(1);

        END.wait().forget();
        END.wait().forget();

        iterations += 1;

        unsafe {
            if X == 0 && Y == 0 {
                println!("Iteration: {}, X and Y == 0", iterations);
            }
        }
    }
}
