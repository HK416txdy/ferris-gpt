use std::ops::{Add,Sub,Mul,Div,AddAssign,SubAssign,MulAssign,DivAssign};
#[derive(Debug,Clone,Copy)]
pub struct Value {
    data: f64
}

impl Value{
    pub fn new(data: f64) -> Self {
        Value { data }
    }

    pub fn repr(&self) -> String {
        format!("Value({})", self.data)
    }
}

impl Add for Value{
    type Output = Value;
    fn add(self, rhs: Self) -> Self::Output {
        Value::new(self.data+rhs.data)
    }

}

impl Sub for Value{
    type Output = Value;
    fn sub(self, rhs: Self) -> Self::Output {
        Value::new(self.data - rhs.data)
    }

}

impl Mul for Value{
    type Output = Value;
    fn mul(self, rhs: Self) -> Self::Output {
        Value::new(self.data * rhs.data)
    }
}

impl Div for Value{
    type Output = Value;
    fn div(self, rhs: Self) -> Self::Output {
        Value::new(self.data / rhs.data)
    }
}

impl AddAssign for Value{
    fn add_assign(&mut self, rhs: Self) {
        self.data += rhs.data;
    }

}

impl SubAssign for Value{
    fn sub_assign(&mut self, rhs: Self) {
        self.data -= rhs.data;
    }

}

impl MulAssign for Value{
    fn mul_assign(&mut self, rhs: Self) {
        self.data *= rhs.data;
    }

}

impl DivAssign for Value{
    fn div_assign(&mut self, rhs: Self) {
        self.data /= rhs.data;
    }

}

impl PartialEq for Value{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }

}