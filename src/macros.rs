macro_rules! regex {
    ($NAME:ident = $pat:literal) => {
        pub static $NAME: std::sync::LazyLock<regex::Regex> =
            std::sync::LazyLock::new(|| regex::Regex::new($pat).unwrap());
    };
}
