#[derive(Default)]
pub struct MyType {
    pub fld1: i32,
    pub fld2: i32,
}

impl MyType {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn update_fld1(mut self, value: i32) -> Self {
        self.fld1 = value;
        self
    }

    pub fn update_fld2(mut self, value: i32) -> Self {
        self.fld2 = value;
        self
    }
}
