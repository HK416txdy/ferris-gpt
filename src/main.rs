use ferris_gpt::value::Value;
fn main() {
    let a = Value::new(3.0);
    let b = Value::new(4.0);
    let c = a+b;
    let d = a+b+c;
    println!("{}", d.repr());
}
