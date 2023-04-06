use std::collections::HashMap;
pub enum Optimizer {
    SGD(SGD),
    Adam(Adam),
    AdaGrad(AdaGrad),
}
pub trait Optimize {
    fn register(&mut self, _index: usize) {
        return ();
    }
    fn optimize(&mut self, index: usize, old_value: f32, grad: f32) -> f32;
}
impl Optimize for Optimizer {
    fn register(&mut self, index: usize) {
        match self {
            Optimizer::SGD(sgd) => sgd.register(index),
            Optimizer::Adam(adam) => adam.register(index),
            Optimizer::AdaGrad(adagrad) => adagrad.register(index),
        }
    }
    fn optimize(&mut self, index: usize, old_value: f32, grad: f32) -> f32 {
        match self {
            Optimizer::SGD(sgd) => return sgd.optimize(index, old_value, grad),
            Optimizer::Adam(adam) => return adam.optimize(index, old_value, grad),
            Optimizer::AdaGrad(adagrad) => return adagrad.optimize(index, old_value, grad),
        }
    }
}

//SGD optimizer
pub struct SGD {
    learning_rate: f32,
}
impl SGD {
    pub fn new(lr: f32) -> Self {
        SGD { learning_rate: lr }
    }
}
impl Optimize for SGD {
    fn register(&mut self, _index: usize) {
        return ();
    }
    fn optimize(&mut self, _index: usize, old_value: f32, grad: f32) -> f32 {
        return old_value - self.learning_rate * grad;
    }
}

//AdaGrad optimizer
pub struct AdaGrad {
    learning_rate: f32,
    aux: HashMap<usize, f32>,
}
impl AdaGrad {
    pub fn new(lr: f32) -> Self {
        AdaGrad {
            learning_rate: lr,
            aux: HashMap::new(),
        }
    }
}
impl Optimize for AdaGrad {
    fn register(&mut self, index: usize) {
        self.aux.entry(index).or_insert(0.0);
    }
    fn optimize(&mut self, index: usize, old_value: f32, grad: f32) -> f32 {
        let h = self.aux.get(&index).unwrap().clone();
        let h2 = h + grad * grad;
        self.aux.insert(index, h2);
        return old_value - self.learning_rate / (h2.sqrt() + 1e-7) * grad;
    }
}

//Adam optimizer
pub struct Adam {
    learning_rate: f32,
    beta0: f32, // 0.9
    beta1: f32, // 0.99
    aux: HashMap<usize, (f32, f32)>,
}
impl Adam {
    pub fn new(lr: f32, b0: f32, b1: f32) -> Self {
        Adam {
            learning_rate: lr,
            beta0: b0,
            beta1: b1,
            aux: HashMap::new(),
        }
    }
}
impl Optimize for Adam {
    fn register(&mut self, index: usize) {
        self.aux.entry(index).or_insert((0.0, 0.0));
    }
    fn optimize(&mut self, index: usize, old_value: f32, grad: f32) -> f32 {
        let (old_vel, old_mov) = self.aux.get(&index).unwrap().clone();
        let vel = self.beta1 * old_vel + (1.0 - self.beta1) * grad;
        let mov = self.beta0 * old_mov + (1.0 - self.beta0) * grad;
        self.aux.insert(index, (vel, mov));
        return old_value - self.learning_rate * mov / (vel.sqrt() + 1e-8);
    }
}
