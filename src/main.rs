trait Printer {
    fn print(&self);
}

struct A {}
impl Printer for A {
    fn print(&self) {
        println!("A");
    }
}
struct B {}
impl Printer for B {
    fn print(&self) {
        println!("B");
    }
}

fn do_print0(p: &impl Printer) {
    p.print();
}

fn do_print1<T: Printer>(p: &T) {
    p.print();
}

fn do_print2<T>(p: &T)
where
    T: Printer,
{
    p.print();
}

fn main() {
    let a = A {};
    let b = B {};
    do_print0(&a);
    do_print0(&b);
    do_print1(&a);
    do_print1(&b);
    do_print2(&a);
    do_print2(&b);
}
