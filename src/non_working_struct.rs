use napi_derive::napi;

#[napi]
struct NonWorkingStruct {
  pub my_field: u32,
}

#[napi]
impl NonWorkingStruct {
  pub fn new(my_field: u32) -> Self {
    NonWorkingStruct { my_field }
  }

  #[napi]
  pub fn get_my_field(&self) -> u32 {
    self.my_field
  }

  #[napi]
  pub fn do_something(&self, my_field: u32) -> u32 {
    self.my_field + my_field
  }
}
