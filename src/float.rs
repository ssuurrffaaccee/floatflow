#[derive(Debug, Clone, Copy)]
pub struct Float {
    pub value: f32,
    pub grad: f32,
    pub is_need_grad: bool,
}

impl Default for Float {
    fn default() -> Self {
        Float {
            value: 0.0,
            grad: 0.0,
            is_need_grad: false,
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn float_build() {
        let f = super::Float::default();
        assert_eq!(f.value, 0.0);
        assert_eq!(f.grad, 0.0);
        assert_eq!(f.is_need_grad, false);
    }
}
