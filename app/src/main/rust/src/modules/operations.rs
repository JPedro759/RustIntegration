
pub fn sum(first: i32, second: i32) -> i32 {
    first + second
}

pub fn sub(first: i32, second: i32) -> i32 {
    first - second
}

pub fn multi(first: i32, second: i32) -> i32 {
    first * second
}

#[derive(Default)]
pub struct DivisionResponse {
    pub value: f32,
    pub status: bool,
}

pub fn div(first: f32, second: f32) -> DivisionResponse {
    if second == 0.0 {
        DivisionResponse { ..Default::default() }
    } else {
        DivisionResponse { value: first / second, status: true }
    }
}
