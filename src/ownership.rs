pub fn take_ownership(s: String) -> usize {
    s.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ownership() {
        let s = String::from("hello");
        assert_eq!(take_ownership(s), 5);
    }

    #[test]
    fn test_ownership_block() {
        let s = {
            let tmp = String::from("hello");
            tmp
        };

        assert_eq!(s, "hello");
    }

    #[test]
    fn test_ownership_mut() {
        let mut x = 10;
        println!("variable x = {}", x);

        x = x + 5;
        println!("after x changed = {}", x);

        assert_eq!(x, 15);
    }
}
