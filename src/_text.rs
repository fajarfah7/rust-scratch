#[cfg(test)]
mod test_text {
    #[test]
    fn test_text_split() {
        let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.\n\n Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.\n\n Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.\n\n Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

        let chunks: Vec<String> = text
            .split("\n\n")
            .inspect(|b| println!("RAW: {:?}", b))
            .map(|b| b.trim())
            .inspect(|b| println!("TRIMMED: {:?}", b))
            .filter(|b| {
                println!("LEN = {}", b.len());
                b.len() > 30
            })
            .map(|b| b.to_string())
            .collect();
    }

    #[test]
    fn test_vec_into_iter() {
        let val: Vec<u16> = vec![1,2,3,4,5,6,7];
        let new_val:Vec<_> = val.into_iter() // harus declare new type Vec<_>
        .filter(|v| *v > 4) // harus set * karena v masih punya vec, ini hasil dari into_iter menghasilkan reference
        .collect(); // signature fn collect<B>(self) -> B where B: FromIterator<Self::Item>; singkatnya ini adalah wadah, belum tau tipenya apa, maka dari itu harus declare Vec<_>
        println!("{:?}", new_val)
    }
}
