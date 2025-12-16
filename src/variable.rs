#[cfg(test)]

mod test_variable {
    use core::*;

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

        let x = x + 1;
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

        let mut x = String::from("hi");

        // mutable borrow
        let y = &mut x; // <- the owner still x
        y.push_str(" world");
        println!("y ==>{}", y);

        // move to z
        let mut z = x; // <- the owner moved to z
        z.push_str(" helloo");
        println!("z ==> {}", z);
        println!("z then ==> {}", z);
        
        assert_eq!(s, "hello world");
    }

    #[test]
    fn test_variable_char() {
        // char use single quote ('), just one char allowed
        let a: char = 'a';
        let b: char = 'b';

        println!("char a = {}, char b = {}", a, b);
    }

    #[test]
    fn test_variable_tuple() {
        let data1: (String, i32, f64, bool) = (String::from("Fajar"), 30, 30.5, true);
        println!("data tuple {:?}", data1);

        let a = data1.0;
        let b = data1.1;
        let c = data1.2;
        let d = data1.3;
        println!("access by index: {}, {}, {}, {}", a, b, c, d);

        // i create new tuple because the value already moved to a, b, c, d
        let data2: (String, i32, f64, bool) = (String::from("Fajar"), 30, 30.5, true);
        let (w, x, y, z) = data2;
        println!("access using destructing: {}, {}, {}, {}", w, x, y, z);

        let data3: (String, i32, f64, bool) = (String::from("Fajar"), 30, 30.5, true);
        let q = &data3.0;
        let r = &data3.1;
        let s = &data3.2;
        let t = &data3.3;
        println!("borrow data from tuple: {}, {}, {}, {}", q, r, s, t);
        let (e, f, g, h) = data3; // <- no error, because borow data
        println!("result form data3: {}, {}, {}, {}", e, f, g, h);

        let data4: (String, i32, f64, bool) = (String::from("Fajar"), 30, 30.5, true);
        let (aa, ab, ac, ad) = &data4;
        println!("borrow data from tuple with destructing: {}, {}, {}, {}", aa, ab, ac, ad);
    }

    // ARRAY the size can't be changed.
    #[test]
    fn test_variable_array() {
        
        let arr: [i32; 3] = [1, 2, 3];
        println!("array {:?}", arr);

        let mut arr_mut: [i32; 3] = [1, 2, 3];
        arr_mut[0] = 10;
        println!("array {:?}", arr_mut);
    }

    // VECTOR the size can grow. common in rust
    #[test]
    fn test_variable_vector() {
        let mut numbers: Vec<i32> = Vec::new();
        numbers.push(1);
        numbers.push(2);
        numbers.push(3);

        let mut v = vec![1,2,3,4];
        v.push(5);

        println!("vector by call Vec<T> ==> {:?}", numbers);
        println!("vector by macro vec![T] ==> {:?}", v);

        fn append_value(mut x: Vec<i32>) -> Vec<i32>{
            for number in 1..=5 {
                x.push(number);
            }
            x
        }

        let mut x: Vec<i32> = Vec::new();
        x = append_value(x);
        println!("vector of x ==>{:?}", x);
    }
}
