pub fn input_string_to_integer_vec(input: &str) -> Vec<i64> {
    input.trim()
        .split("\n")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}
pub fn input_string_to_str_vec(input: &str) -> Vec<&str> {
    input.trim()
        .split("\n")
        .collect::<Vec<&str>>()
}
