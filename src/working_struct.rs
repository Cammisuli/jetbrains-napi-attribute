struct MyWorkingStruct {
    pub my_field: u32,
}

impl MyWorkingStruct {
    pub fn new(my_field: u32) -> Self {
        MyWorkingStruct { my_field }
    }

    pub fn get_my_field(&self) -> u32 {
        self.my_field
    }

    pub fn do_something(&self, my_field: u32) -> u32 {
        self.my_field + my_field
    }
}
