use floatflow::graph::NodeAllocator;
use floatflow::polynomial::{Polynomial, Term};
use std::collections::HashMap;
fn main() {
    let mut poly = Polynomial::new();
    {
        let mut t = Term::new();
        t.set_coefficient(1.0);
        t.add_variable("x", 2);
        println!("{:?}", t);
        poly.add_term(t);
    }
    {
        let mut t = Term::new();
        t.set_coefficient(1.0);
        println!("{:?}", t);
        poly.add_term(t);
    }
    println!("{:?}", poly);
    let mut allocator = NodeAllocator::new();
    let mut name_record = HashMap::<String, usize>::new();
    let graph = poly.tranform_to_graph(&mut name_record, &mut allocator);
    println!("{:?}", graph);
}
