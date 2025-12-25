// "lifetime" ensure that the owner MUST alive until borrow is done.

// // lifetime
// fn get_str() -> &str {
//     let s = String::from("hello");
//     &s
// }

fn get_str<'a>(s: &'a str) -> &'a str {
    s
}

#[derive(Debug)]
struct User<'a> {
    name: &'a str,
}

impl<'a>User<'a> {
    fn name(&self) -> &'a str {
        self.name
    }
}

#[cfg(test)]
mod test_lifetime {
    use crate::_lifetime::{User, get_str};

    #[test]
    fn test_get_str() {
        let owner = "i am owner";
        let s = get_str(owner);
        println!("{}", s)
    }

    #[test]
    fn test_struct_user() {
        let name = "test";
        let user = User { name: name };
        let my_name = user.name();
        println!("{}", user.name);
        println!("{:?}", user);
        println!("my name: {}", my_name);
        println!("{}", user.name);
    }
}
