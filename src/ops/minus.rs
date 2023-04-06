use crate::error::{FFError, Result};
use crate::float::Float;
use crate::store::Store;
use std::cell::RefCell;
pub struct Minus(usize, usize);
impl Minus {
    pub fn new(r: usize, res: usize) -> Self {
        Minus(r, res)
    }
    pub fn forward(&self, s: &mut Store) -> Result<()> {
        let mut res = Float::default();
        {
            let r = s
                .get(&self.0)
                .ok_or(FFError::Store(format!("not find id: {}", self.0)))?
                .borrow();
            if r.is_need_grad {
                res.is_need_grad = true;
            }
            res.value = -1.0 * r.value;
            res.grad = 0.0;
        }
        s.insert(self.1, RefCell::new(res));
        return Ok(());
    }
    pub fn backward(&self, s: &mut Store) -> Result<()> {
        let mut r = s
            .get(&self.0)
            .ok_or(FFError::Store(format!("not find id: {}", self.0)))?
            .borrow_mut();
        let res = s
            .get(&self.1)
            .ok_or(FFError::Store(format!("not find id: {}", self.1)))?
            .borrow();
        if r.is_need_grad {
            r.grad += -1.0 * res.grad;
        }
        return Ok(());
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minus() {
        let mut s = Store::new();
        {
            let mut r = Float::default();
            r.value = 1.0;
            r.is_need_grad = true;
            s.insert(0, RefCell::new(r));
        }
        let minus = Minus(0, 1);
        minus.forward(&mut s);
        {
            let mut res = s.get(&1).unwrap().borrow_mut();
            res.grad = 1.0;
        }
        minus.backward(&mut s);
        let r = s.get(&0).unwrap().borrow();
        let res = s.get(&1).unwrap().borrow();
        assert_eq!(res.value, -1.0);
        assert_eq!(res.is_need_grad, true);
        assert_eq!(res.grad, 1.0);
        assert_eq!(r.is_need_grad, true);
        assert_eq!(r.grad, -1.0);
    }
}
