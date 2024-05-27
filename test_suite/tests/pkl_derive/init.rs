use pkl_derive::Pkl;
use pkl_bind::evaluator::decoder::Pkl;

#[derive(Pkl)]
struct Tests {
    an_int: i64,
    another_int: i32,
    a_float: f32,
}

fn main() {}
