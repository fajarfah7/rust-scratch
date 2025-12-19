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
}
