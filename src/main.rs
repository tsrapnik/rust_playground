use heapless::{pool, pool::singleton::Pool};
use static_cell::StaticCell;
use std::thread;

fn main() {
    pool!(A: [u32; 16]);
    const SIZE: usize = (4 * 16 + 8) * 10;
    static mut A_BUFFER: [u8; SIZE] = [0; SIZE];
    unsafe {
        A::grow(&mut A_BUFFER);
    }

    for _ in 0..10 {
        let a = A::alloc().unwrap().init([99; 16]);
        thread::spawn(move || println!("{a:?}"));
    }

    static SC: StaticCell<u32> = StaticCell::new();
    let sc = SC.init(8);
    println!("{sc}");
}
