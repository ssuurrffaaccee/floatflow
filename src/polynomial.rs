use crate::graph::{self, Graph, NodeAllocator};
use std::collections::HashMap;
#[derive(Debug)]
pub struct Term {
    coefficient: f32,
    variables: HashMap<String, usize>,
}
impl Term {
    pub fn new() -> Self {
        Term {
            coefficient: 1.0,
            variables: HashMap::new(),
        }
    }
    pub fn set_coefficient(&mut self, cof: f32) {
        self.coefficient = cof;
    }
    pub fn add_variable(&mut self, name: &str, order: usize) {
        self.variables
            .entry(name.to_string())
            .and_modify(|e| *e = *e + order)
            .or_insert(order);
    }
}
#[derive(Debug)]
pub struct Polynomial {
    terms: Vec<Term>,
}
impl Polynomial {
    pub fn new() -> Self {
        Polynomial { terms: Vec::new() }
    }
    pub fn add_term(&mut self, t: Term) {
        self.terms.push(t);
    }
    pub fn tranform_to_graph(
        &self,
        name_to_index: &mut HashMap<String, usize>,
        allocator: &mut NodeAllocator,
    ) -> Graph {
        let mut g = Graph::new();
        let output = tranform_polynomial_to_graph(self, &mut g, name_to_index, allocator);
        let loss = g.mul(output, output, allocator);
        g.output(loss);
        return g;
    }
}

fn tranform_polynomial_to_graph(
    poly: &Polynomial,
    g: &mut Graph,
    name_to_index: &mut HashMap<String, usize>,
    allocator: &mut NodeAllocator,
) -> usize {
    let term_results = poly
        .terms
        .iter()
        .map(|t| {
            let mut var_exps = t
                .variables
                .iter()
                .map(|(name, order)| {
                    //一个坑: 下面语句中的g.paramter一定会被调用,在entry中key存在时也会被调用:(
                    //let var = name_to_index.entry(name.to_string()).or_insert(g.parameter(name));
                    let var = name_to_index
                        .entry(name.to_string())
                        .or_insert_with(|| g.parameter(name, allocator));
                    return graph::binary_exponentiation(*var, *order, g, allocator);
                })
                .collect::<Vec<usize>>();
            if t.coefficient != 1.0 || var_exps.len() == 0 {
                let cof = g.constant(t.coefficient, allocator);
                var_exps.push(cof);
            }
            return graph::mul(&var_exps, g, allocator);
        })
        .collect::<Vec<usize>>();
    return graph::sum(&term_results, g, allocator);
}
