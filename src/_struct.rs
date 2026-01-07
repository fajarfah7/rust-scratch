#[cfg(test)]

mod test_struct {
    use core::*;
    
    #[test]
    fn test_struct_declaration() {
        // #[derive(Debug)]
        struct User {
            name: String,
            age: u32,
            active: bool,
            score: f32,
            password: String,
        }

        impl fmt::Debug for User {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("User")
                .field("name", &self.name)
                .field("age", &self.age)
                .field("active", &self.active)
                .field("score", &self.score)
                .field("password", &"***")
                .finish()
            }
        }

        let user = User {
            name: String::from("Fajar"),
            age: 30,
            active: true,
            score: 9.7,
            password: "password".to_string(),
        };
        println!("{:#?}", user); // use pretty print

        assert_eq!(user.name, "Fajar");
        assert_eq!(user.age, 30);
        assert_eq!(user.active, true);
        assert_eq!(user.score, 9.7);
        assert_eq!(user.password, "password");

        let mut mut_user = User {
            name: String::from("Fathian"),
            age: 2,
            active: true,
            score: 9.8,
            password: "password".to_string(),
        };
        mut_user.score = 9.9;

        println!("{:?}", mut_user);
    }

    #[test]
    fn test_struct_tuple() {
        struct Point(f64, f64, f64);

        let p: Point = Point(0.2, 0.4, 0.3);
        println!("X: {}", p.0);
        println!("Y: {}", p.1);
        println!("Z: {}", p.2);
    }

    #[test]
    fn test_struct_generic() {
        struct Const<T> {
            value: T,
        }

        impl<T> Const<T> {
            fn set(value: T) -> Self {
                Self { value }
            }
            fn get(&self) -> &T {
                &self.value
            }
        }

        let phi = Const::<f64>::set(3.14);
        println!("Phi: {}", phi.get());

        let g = Const::<f64>::set(9.8);
        println!("G: {}", g.get());
    }
}
