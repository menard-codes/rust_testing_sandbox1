pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn it_adds() {
        assert_eq!(add(4, 5), 9);
    }
}
