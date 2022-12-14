pub fn string_to_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
pub fn bool_to_str(val: &bool) -> &'static str {
    return match val {
        true => "true",
        false => "false",
    };
}
