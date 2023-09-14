#[inline(never)]
fn x(value: &str) -> usize {
    if value == "x" {
        panic!("panicked in x");
    }
    value.len()
}

#[inline(never)]
fn bar(value: &str) -> usize {
    if value == "bar" {
        panic!("panicked in bar");
    }
    x(value) + 1
}

#[inline(never)]
fn foo(value: &str) -> usize {
    if value == "foo" {
        panic!("panicked in foo");
    }
    bar(value) + 1
}

fn main() {
    install_panic_hook();

    let args: Vec<String> = std::env::args().collect();

    // do not check length, so that we panic if there is no argument
    let result = foo(&args[1]);
    println!("result = {result}");
}

fn install_panic_hook() {
    std::panic::set_hook(Box::new(move |info| {
        let backtrace = backtrace::Backtrace::default();

        let message = if let Some(s) = info.payload().downcast_ref::<&'static str>() {
            *s
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            s
        } else {
            "unknown panic info type"
        };

        match info.location() {
            Some(location) => {
                println!(
                    "panicked at '{}': {}:{}{:?}",
                    message,
                    location.file(),
                    location.line(),
                    backtrace,
                );
            }
            None => println!("panicked at '{}'{:?}", message, backtrace),
        }
    }));
}
