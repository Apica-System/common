pub struct ValueNull {

}

impl ValueNull {
    pub fn init() -> ValueNull {
        return ValueNull{};
    }
    
    pub fn show(&self, end: char) {
        print!("null{end}");
    }
    
    pub fn is_null(&self) -> bool {
        return true;
    }
    
    pub fn get_type_representation(&self) -> &str {
        return "null";
    }
}