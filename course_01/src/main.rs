mod l1_loop;
mod l2_loop;

fn main() {
    println!("L1 submod print [a..Z]");
    l1_loop::print();
    
    println!("\nL2 submod print [A..z]");
    l2_loop::print::print();
}
