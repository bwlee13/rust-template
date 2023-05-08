
#[cfg(test)]
mod tests {
    use utils::add_one;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}