// function item v.s. function pointer
// Fn, FnMut, FnOnce traits.
// function pointers impl all three traits.

// closure: non-capturing closure, and capturing closure.
fn main() {
    println!("Hello, world!");
}

fn quox<F>(f: F)
where
    F: Fn(),
{
    (f)()
}
