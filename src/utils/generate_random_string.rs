pub fn generate_random_string() -> String {
    return std::iter::repeat_with(fastrand::alphanumeric)
        .take(16)
        .collect();
}
