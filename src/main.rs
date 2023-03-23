use std::io;

fn main() {
    println!("{:?}", return_error_or_ok(true));
    println!("{:?}", return_error_or_ok(false));
    println!("{:?}", return_some_or_none(true));
    println!("{:?}", return_some_or_none(false));
}

fn return_some_or_none(some: bool) -> Option<u32> {
    return_error(some).ok()
}
fn return_error_or_ok(ok: bool) -> Result<u32, ()> {
    return_error(ok).map_err(|_| ())
}

fn return_error(error: bool) -> Result<u32, io::Error> {
    if error {
        Ok(5)
    } else {
        Err(io::Error::new(io::ErrorKind::AddrInUse, "io error"))
    }
}
