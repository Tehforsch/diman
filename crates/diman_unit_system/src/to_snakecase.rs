use proc_macro2::Ident;

fn str_to_snakecase(s: &str) -> String {
    let s = s.chars().rev().collect::<String>();
    let words = s.split_inclusive(|c: char| c.is_uppercase());
    words
        .map(|word| word.chars().rev().collect::<String>().to_lowercase())
        .rev()
        .collect::<Vec<_>>()
        .join("_")
}

pub fn to_snakecase(dim: &Ident) -> Ident {
    let snake_case = str_to_snakecase(&dim.to_string());
    Ident::new(&snake_case, dim.span())
}

#[cfg(test)]
mod tests {
    #[test]
    fn str_to_snakecase() {
        assert_eq!(super::str_to_snakecase("MyType"), "my_type".to_owned());
        assert_eq!(super::str_to_snakecase("My"), "my".to_owned());
        assert_eq!(
            super::str_to_snakecase("MyVeryLongType"),
            "my_very_long_type".to_owned()
        );
    }
}
