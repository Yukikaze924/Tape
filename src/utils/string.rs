pub trait BooleanIdentifiable {
    fn is_bool_string(&self) -> bool;
}

impl BooleanIdentifiable for String {
    fn is_bool_string(&self) -> bool {
        return self == "true" || self == "false";
    }
}

impl BooleanIdentifiable for str {
    fn is_bool_string(&self) -> bool {
        return self == "true" || self == "false";
    }
}