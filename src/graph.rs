use crate::op::Op;
use std::collections::HashMap;
#[derive(Debug)]
pub struct Graph {
    ops: Vec<Op>,
    inputs: HashMap<String, usize>,
    parameters: HashMap<String, usize>,
    constants: HashMap<usize, f32>,
    output: usize,
}
pub struct NodeAllocator {
    counter: usize,
}
impl NodeAllocator {
    pub fn new() -> Self {
        NodeAllocator { counter: 0 }
    }
    pub fn alloc_index(&mut self) -> usize {
        self.counter += 1;
        self.counter
    }
}
impl Graph {
    pub fn new() -> Self {
        Graph {
            ops: Vec::new(),
            inputs: HashMap::new(),
            parameters: HashMap::new(),
            constants: HashMap::new(),
            output: 0,
        }
    }
    pub fn get_ops(&self) -> &Vec<Op> {
        return self.ops.as_ref();
    }
    pub fn get_inputs(&self) -> &HashMap<String, usize> {
        let ref ref_v = self.inputs;
        return ref_v;
    }
    pub fn get_parameters(&self) -> &HashMap<String, usize> {
        let ref ref_v = self.parameters;
        return ref_v;
    }
    pub fn get_constants(&self) -> &HashMap<usize, f32> {
        let ref ref_v = self.constants;
        return ref_v;
    }
    pub fn get_output(&self) -> usize {
        return self.output;
    }
    pub fn input(&mut self, name: &str, allocator: &mut NodeAllocator) -> usize {
        let i = allocator.alloc_index();
        self.inputs.insert(name.to_string(), i);
        return i;
    }
    pub fn parameter(&mut self, name: &str, allocator: &mut NodeAllocator) -> usize {
        let p = allocator.alloc_index();
        self.parameters.insert(name.to_string(), p);
        return p;
    }
    pub fn add(&mut self, r: usize, l: usize, allocator: &mut NodeAllocator) -> usize {
        let res = allocator.alloc_index();
        self.ops.push(Op::Add(r, l, res));
        return res;
    }
    pub fn minus(&mut self, r: usize, allocator: &mut NodeAllocator) -> usize {
        let res = allocator.alloc_index();
        self.ops.push(Op::Minus(r, res));
        return res;
    }
    pub fn mul(&mut self, r: usize, l: usize, allocator: &mut NodeAllocator) -> usize {
        let res = allocator.alloc_index();
        self.ops.push(Op::Mul(r, l, res));
        return res;
    }
    pub fn output(&mut self, r: usize) {
        self.output = r;
    }
    pub fn constant(&mut self, constant: f32, allocator: &mut NodeAllocator) -> usize {
        let constant_index = allocator.alloc_index();
        self.constants.insert(constant_index, constant);
        return constant_index;
    }
}

pub fn sum(vs: &Vec<usize>, g: &mut Graph, allocator: &mut NodeAllocator) -> usize {
    let mut res = vs[0];
    vs.iter()
        .skip(1)
        .for_each(|v| res = g.add(res, *v, allocator));
    return res;
}
pub fn mul(vs: &Vec<usize>, g: &mut Graph, allocator: &mut NodeAllocator) -> usize {
    let mut res = vs[0];
    vs.iter()
        .skip(1)
        .for_each(|v| res = g.mul(res, *v, allocator));
    return res;
}
pub fn binary_exponentiation_old(
    var: usize,
    order: usize,
    g: &mut Graph,
    allocator: &mut NodeAllocator,
) -> usize {
    let mut order_to_compute = order;
    if order_to_compute < 2 {
        return var;
    }
    let mut res_for_tree = Vec::new();
    while order_to_compute != 0 {
        let mut computed_order = 1;
        let mut res = var;
        loop {
            if computed_order * 2 > order_to_compute {
                order_to_compute = order_to_compute - computed_order;
                break;
            }
            res = g.mul(res, res, allocator);
            computed_order *= 2;
        }
        res_for_tree.push(res);
    }
    return mul(&res_for_tree, g, allocator);
}
pub fn binary_exponentiation(
    var: usize,
    order: usize,
    g: &mut Graph,
    allocator: &mut NodeAllocator,
) -> usize {
    if order == 0 {
        return g.constant(1.0, allocator);
    }
    if order == 1 {
        return var;
    }
    let mut order_to_compute = order;
    let mut max_power = 1;
    let mut mem = Vec::new();
    mem.resize((order as f32).log(2.0) as usize + 1, 0);
    mem[0] = var;
    let mut res = var;
    while order_to_compute != 1 {
        res = g.mul(res, res, allocator);
        mem[max_power] = res;
        max_power += 1;
        order_to_compute /= 2
    }
    for offset in 0..max_power - 1 {
        if (order >> offset & 1usize) == 1 {
            res = g.mul(res, mem[offset], allocator);
        }
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn graph() {
        let mut g = Graph::new();
        let mut allocator = NodeAllocator::new();
        let x = g.input("x", &mut allocator); // index 1;
        let y = g.input("y", &mut allocator); // index 2;
        let z = g.add(x, y, &mut allocator); // index 3;
        let m = g.minus(z, &mut allocator); // index 4
        let p = g.parameter("p", &mut allocator); //index 5
        let f = g.mul(m, p, &mut allocator); // index 6
        g.output(f);

        let ops = g.get_ops();
        let ref_ops = vec![Op::Add(1, 2, 3), Op::Minus(3, 4), Op::Mul(4, 5, 6)];
        assert_eq!(ops.iter().zip(ref_ops.iter()).all(|(r, l)| *r == *l), true);
        let inputs = g.get_inputs();
        assert_eq!(*inputs.get("x").unwrap(), 1);
        assert_eq!(*inputs.get("y").unwrap(), 2);
        let parameters = g.get_parameters();
        assert_eq!(*parameters.get("p").unwrap(), 5);
        assert_eq!(g.get_output(), 6);
    }
}
