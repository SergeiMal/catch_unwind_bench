use criterion::{black_box, criterion_group, criterion_main, Criterion};

use std::panic;

// It need just to calculate the step
fn next_step_fib(n: u16) -> u64 {
    let mut x = (1, 1);
    for _ in 0..n {
        x = (x.1, x.0 + x.1)
    }

    x.0
}

// Simple recursion without panics and catching of them
fn simply_fn(deep: u16) {
    if deep > 0 {
        let _d = deep * 10;
        simply_fn(deep - 1);
    }
}

// Simple recursion without catching of panic, but it panics at the depth
fn simply_fn_panic_bottom(deep: u16) {
    if deep > 0 {
        let _d = deep * 10;
        simply_fn_panic_bottom(deep - 1);
    }else {
        panic!("oh no!");
    }
}

// Recursion with the catching at the top, and without through panics
fn catch_panic_at_top(deep: u16) {
    if deep > 0 {
        let _d = deep * 10;
        let _p = panic::catch_unwind(|| simply_fn(deep - 1));
    }
}

// Recursion with the catching on every layer, and without through panics
fn catch_panic(deep: u16) {
    if deep > 0 {
        let _d = deep * 10;
        let _p = panic::catch_unwind(|| catch_panic(deep - 1));
    }
}

// Recursion with the catching at the top, and through a panic at the depth
fn catch_at_top_panic_at_bottom(deep: u16) {
    if deep > 0 {
        let _d = deep * 10;
        let _p = panic::catch_unwind(|| simply_fn_panic_bottom(deep - 1));
    }
}

// Recursion with the catching on every layer, and through a panic at the depth
fn catch_panic_trow_once(deep: u16) {
    if deep > 0 {
        let _d = deep * 10;
        let _p = panic::catch_unwind(|| catch_panic_trow_once(deep - 1));
    } else {
        panic!("oh no!");
    }
}

// Catch panics at every layer and resume it, and throw a panic at the depth
fn catch_panic_trow(deep: u16) {
    if deep > 0 {
        let _d = deep * 10;
        let result = panic::catch_unwind(|| catch_panic_trow(deep - 1));
        match result {
            Err(panic) => panic::resume_unwind(panic),
            _ => panic!("not an error?"),
        }
    } else {
        panic!("oh no!");
    }
}

// A wrapper to call  catch_panic_trow
fn catch_panic_trow_call(deep: u16) {
    if deep > 0 {
        let _d = deep * 10;
        let _p = panic::catch_unwind(|| catch_panic_trow(deep - 1));
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    panic::set_hook(Box::new(|e| {
        black_box(e);
    }));

    for n in 0..=12 {
        let deep = next_step_fib(n) as u16;
        c.bench_function(format!("Simply fn. depth {}", deep).as_str(),
                         |b| b.iter(|| simply_fn(black_box(deep))));
        c.bench_function(format!("Catch panic at top. depth {}", deep).as_str(),
                         |b| b.iter(|| catch_panic_at_top(black_box(deep))));
        c.bench_function(format!("Catch panic. depth {}", deep).as_str(),
                         |b| b.iter(|| catch_panic(black_box(deep))));
        c.bench_function(format!("Catch panic at top, throw at bottom. depth {}", deep).as_str(),
                         |b| b.iter(|| catch_at_top_panic_at_bottom(black_box(deep))));
        c.bench_function(format!("Catch panic throw once. depth {}", deep).as_str(),
                         |b| b.iter(|| catch_panic_trow_once(black_box(deep))));
        c.bench_function(format!("Catch panic throw. depth {}", deep).as_str(),
                         |b| b.iter(|| catch_panic_trow_call(black_box(deep))));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);