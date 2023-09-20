#[derive(Debug, Copy, Clone)]
pub enum JvmValue {
    Int(i32),
    Long(i64),

    Float(f32),
    Double(f64),

    Reference(usize),

    Undefined,
}