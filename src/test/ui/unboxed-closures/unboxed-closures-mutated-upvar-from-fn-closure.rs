// revisions: ast mir
//[mir]compile-flags: -Z borrowck=mir

// Test that a by-ref `FnMut` closure gets an error when it tries to
// mutate a value.

fn call<F>(f: F) where F : Fn() {
    f();
}

fn main() {
    let mut counter = 0;
    call(|| {
        counter += 1;
        //[ast]~^ ERROR cannot assign to data in a captured outer variable in an `Fn` closure
        //[mir]~^^ ERROR cannot assign to `counter`
    });
}
