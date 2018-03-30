mod print_loop {
    pub fn foo() {
        for i in 1..10 {
            println!("{}", i);
        }
    }
}

fn main() {
    print_loop::foo();
}
