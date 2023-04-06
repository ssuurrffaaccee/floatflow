use crate::error::{FFError, Result};
use crate::float::Float;
use crate::store::Store;
use std::cell::RefCell;
pub struct Add(usize, usize, usize);
impl Add {
    pub fn new(r: usize, l: usize, res: usize) -> Self {
        Add(r, l, res)
    }
    pub fn forward(&self, s: &mut Store) -> Result<()> {
        let mut res = Float::default();
        {
            let r = s
                .get(&self.0)
                .ok_or(FFError::Store(format!("not find id: {}", self.0)))?
                .borrow();
            let l = s
                .get(&self.1)
                .ok_or(FFError::Store(format!("not find id: {}", self.1)))?
                .borrow();
            if r.is_need_grad || l.is_need_grad {
                res.is_need_grad = true;
            }
            res.value = r.value + l.value;
            res.grad = 0.0;
        }
        s.insert(self.2, RefCell::new(res));
        return Ok(());
    }
    pub fn backward(&self, s: &mut Store) -> Result<()> {
        if self.0 == self.1 {
            let mut r = s
                .get(&self.0)
                .ok_or(FFError::Store(format!("not find id: {}", self.0)))?
                .borrow_mut();
            let res = s
                .get(&self.2)
                .ok_or(FFError::Store(format!("not find id: {}", self.2)))?
                .borrow();
            if r.is_need_grad {
                r.grad += 2.0 * res.grad;
            }
        } else {
            let mut r = s
                .get(&self.0)
                .ok_or(FFError::Store(format!("not find id: {}", self.0)))?
                .borrow_mut();
            let mut l = s
                .get(&self.1)
                .ok_or(FFError::Store(format!("not find id: {}", self.1)))?
                .borrow_mut();
            let res = s
                .get(&self.2)
                .ok_or(FFError::Store(format!("not find id: {}", self.2)))?
                .borrow();
            if r.is_need_grad {
                r.grad += res.grad;
            }
            if l.is_need_grad {
                l.grad += res.grad;
            }
        }
        return Ok(());
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add() {
        let mut s = Store::new();
        {
            let mut r = Float::default();
            r.value = 1.0;
            r.is_need_grad = true;
            s.insert(0, RefCell::new(r));
        }
        {
            let mut l = Float::default();
            l.value = 2.0;
            l.is_need_grad = true;
            s.insert(1, RefCell::new(l));
        }
        let mul = Add(0, 1, 2);
        mul.forward(&mut s);
        {
            let mut res = s.get(&2).unwrap().borrow_mut();
            res.grad = 1.0;
        }
        mul.backward(&mut s);
        let r = s.get(&0).unwrap().borrow();
        let l = s.get(&1).unwrap().borrow();
        let res = s.get(&2).unwrap().borrow();
        assert_eq!(res.value, 3.0);
        assert_eq!(res.is_need_grad, true);
        assert_eq!(res.grad, 1.0);
        assert_eq!(r.is_need_grad, true);
        assert_eq!(l.is_need_grad, true);
        assert_eq!(r.grad, 1.0);
        assert_eq!(l.grad, 1.0);
    }
    #[test]
    fn add_self() {
        let mut s = Store::new();
        {
            let mut r = Float::default();
            r.value = 1.0;
            r.is_need_grad = true;
            s.insert(0, RefCell::new(r));
        }
        let mul = Add(0, 0, 1);
        mul.forward(&mut s);
        {
            let mut res = s.get(&1).unwrap().borrow_mut();
            res.grad = 1.0;
        }
        mul.backward(&mut s);
        let r = s.get(&0).unwrap().borrow();
        let res = s.get(&1).unwrap().borrow();
        assert_eq!(res.value, 2.0);
        assert_eq!(res.is_need_grad, true);
        assert_eq!(res.grad, 1.0);
        assert_eq!(r.is_need_grad, true);
        assert_eq!(r.grad, 2.0);
    }
}
