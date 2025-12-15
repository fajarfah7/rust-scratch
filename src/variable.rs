#[cfg(test)]

mod test_variable{
    #[test]
    fn test_variable_declare() {
        let x = 10;
        println!("the value of x = {}", x);
        assert_eq!(x, 10);
    }

    #[test]
    fn test_variable_shadowing() {
        let x = 5;
        println!("value of x {}", x);

        let x = x+1;
        println!("after shadowing {}", x);

        assert_eq!(x, 6);
    }

    #[test]
    fn test_variable_shadowing_change_type() {
        let x = "hello";
        println!("x as &str = {}", x);

        let x = x.len();
        println!("x as usize = {}", x);

        assert_eq!(x, 5);
    }

    #[test]
    fn test_variable_shadowing_scope() {
        let x = 10;
        {
            let x = x + 20;
            println!("x in the block = {}", x);
            assert_eq!(x, 30);
        }
        
        println!("x outside the block = {}", x);

        assert_eq!(x, 10);
    }

    #[test]
    fn test_variable_string_immutable() {
        let s: &str = "hello"; // immutable, can not be changed (usage: parameter, config, constant, etc).
        let t = String::from("world");

        let a = "test".to_string();
        // a = "hello".to_string(); // error
        let b = String::from("test");
        // b = "world".to_string(); // error
        let c: &str = &a;

        println!("a ==> {}", a);
        println!("b ==> {}", b);
        println!("c ==> {}", c);

        let mut x: String = b;
        x.push_str("123");
        println!("x  ==> {}", x);
        // println!("b then ==> {}", b); // error b moved to x

        let y = &c;
        println!("y  ==> {}", y);
        println!("c  ==> {}", c); // no error because y borrow from c, c still owner here.

        let mut d = String::from("test");
        let z = &mut d;
        println!("z borrow d ==> {}", z); // owner still d
        // println!("d  ==> {}", d); // error because d on next line is in borrow.
        z.push_str(" 123");
        println!("z after  ==> {}", z); // borrow process by z done, d is free can be printed. if you switch line with below it will be error.
        println!("d after  ==> {}", d);
        
        assert_eq!(s, "hello");
        assert_eq!(t, "world");
    }

    #[test]
    fn test_variable_string_mutable() {
        let mut s = String::from("hello"); // owned/mutable.
        s.push_str(" world");

        println!("variable s = {}", s);

        assert_eq!(s, "hello world");
    }
}