#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

impl CalculatorInput {
    fn compute(&self, a: i32, b:i32) -> i32 {
        match self {
            &Self::Add => a + b,
            &Self::Subtract => a - b,
            &Self::Multiply => a * b,
            &Self::Divide => a / b,
            _ => panic!()
        }
    }
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut values = Vec::with_capacity(3);
    for i in inputs {
        match i {
            &CalculatorInput::Value(n) => {values.push(n);},
            op if values.len() >1 => {
                let (a, b) = (values.pop().unwrap(), values.pop().unwrap());
                values.push(op.compute(b, a));
            },
            _ => {return None}
        }
    }

    if values.len() == 1 {
        return Some(values[0])
    }
    return None
}
