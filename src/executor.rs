use crate::error::{FFError, Result};
use crate::float::Float;
use crate::graph::Graph;
use crate::op::Op;
use crate::ops;
use crate::optimizer::{AdaGrad, Adam, Optimize, Optimizer, SGD};
use crate::store::Store;
use std::cell::RefCell;
use std::collections::HashMap;
pub struct Executor {
    graph: Graph,
    optimizer: Optimizer,
}
impl Executor {
    pub fn new(g: Graph, opt: Optimizer) -> Self {
        Executor {
            graph: g,
            optimizer: opt,
        }
    }
    pub fn forward(&mut self, s: &mut Store) -> Result<()> {
        for op in self.graph.get_ops().iter() {
            match op {
                Op::Add(r, l, res) => ops::Add::new(*r, *l, *res).forward(s)?,
                Op::Minus(r, res) => ops::Minus::new(*r, *res).forward(s)?,
                Op::Mul(r, l, res) => ops::Mul::new(*r, *l, *res).forward(s)?,
            }
        }
        return Ok(());
    }
    pub fn backward(&mut self, s: &mut Store) -> Result<()> {
        {
            let output_id = self.graph.get_output();
            let mut output = s
                .get(&output_id)
                .ok_or(FFError::Store(format!("not find id: {}", output_id)))?
                .borrow_mut();
            output.grad = 1.0;
        }
        for op in self.graph.get_ops().iter().rev() {
            match op {
                Op::Add(r, l, res) => ops::Add::new(*r, *l, *res).backward(s)?,
                Op::Minus(r, res) => ops::Minus::new(*r, *res).backward(s)?,
                Op::Mul(r, l, res) => ops::Mul::new(*r, *l, *res).backward(s)?,
            }
        }
        return Ok(());
    }
    pub fn feed(&mut self, input_values: &HashMap<String, f32>, s: &mut Store) -> Result<()> {
        for (k, index) in self.graph.get_inputs().iter() {
            let value = input_values
                .get(k)
                .ok_or(FFError::Store(format!("not find key: {}", k)))?;
            let mut float_value = Float::default();
            float_value.is_need_grad = false;
            float_value.value = *value;
            s.insert(*index, RefCell::new(float_value));
        }
        return Ok(());
    }
    pub fn init_parameter(
        &mut self,
        input_values: &HashMap<String, f32>,
        s: &mut Store,
    ) -> Result<()> {
        for (k, index) in self.graph.get_parameters().iter() {
            let value = input_values
                .get(k)
                .ok_or(FFError::Store(format!("not find key: {}", k)))?;
            let mut float_value = Float::default();
            float_value.is_need_grad = true;
            float_value.value = *value;
            s.insert(*index, RefCell::new(float_value));
            self.optimizer.register(*index);
        }
        return Ok(());
    }
    pub fn init_contants(&mut self, s: &mut Store) {
        for (index, value) in self.graph.get_constants().iter() {
            let mut float_value = Float::default();
            float_value.is_need_grad = false;
            float_value.value = *value;
            s.insert(*index, RefCell::new(float_value));
        }
    }
    pub fn clear_grad(&mut self, s: &mut Store) -> Result<()> {
        for (k, index) in self.graph.get_parameters().iter() {
            s.get(&index)
                .ok_or(FFError::Store(format!("not find id: {}", k)))?
                .borrow_mut()
                .grad = 0.0;
        }
        return Ok(());
    }
    pub fn optimize(&mut self, s: &mut Store) -> Result<()> {
        for (_, index) in self.graph.get_parameters().iter() {
            let mut f = s
                .get(index)
                .ok_or(FFError::Store(format!("not find id: {}", index)))?
                .borrow_mut();
            f.value = self.optimizer.optimize(*index, f.value, f.grad);
        }
        return Ok(());
    }

    pub fn get_output_value(&self, s: &mut Store) -> Result<Float> {
        let ouput_id = self.graph.get_output();
        return Ok(*(s
            .get(&ouput_id)
            .ok_or(FFError::Store(format!("not find id: {}", ouput_id)))?
            .borrow()));
    }

    pub fn get_value(&self, index: usize, s: &mut Store) -> Result<Float> {
        return Ok(*(s
            .get(&index)
            .ok_or(FFError::Store(format!("not find id: {}", index)))?
            .borrow()));
    }

    pub fn get_parameters(&self, s: &mut Store) -> Vec<Result<(String, Float)>> {
        return self
            .graph
            .get_parameters()
            .iter()
            .map(|(name, index)| {
                Ok((
                    name.clone(),
                    s.get(index)
                        .ok_or(FFError::Store(format!("not find id: {}", index)))?
                        .borrow()
                        .clone(),
                ))
            })
            .collect::<Vec<Result<(String, Float)>>>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::NodeAllocator;
    #[test]
    fn executor() {
        let mut allocator = NodeAllocator::new();
        //build graph
        let mut g = Graph::new();
        let x = g.input("x", &mut allocator); // index 1;
        let y = g.input("y", &mut allocator); // index 2;
        let z = g.add(x, y, &mut allocator); // index 3;
        let m = g.minus(z, &mut allocator); // index 4
        let p = g.parameter("p", &mut allocator); //index 5
        let f = g.mul(m, p, &mut allocator); // index 6
        g.output(f);

        let mut s = Store::new();
        let mut exec = Executor::new(g, Optimizer::SGD(SGD::new(0.001)));
        exec.feed(
            &HashMap::from([("x".to_string(), 0.1), ("y".to_string(), 0.2)]),
            &mut s,
        );
        exec.init_parameter(&HashMap::from([("p".to_string(), 0.1)]), &mut s);
        exec.init_contants(&mut s);
        exec.forward(&mut s);
        exec.backward(&mut s);
        let output_float = exec.get_output_value(&mut s).unwrap();
        assert!((output_float.value - (-0.03)).abs() < 0.00001);
        assert!(output_float.grad - 1.0 < 0.00001);
        let parameter_p = exec.get_value(p, &mut s).unwrap();
        assert!(parameter_p.value - 0.1 < 0.00001);
        assert!(parameter_p.grad - (-0.3) < 0.00001);
    }
}