pub mod greetings
{
    pub mod english
    {
        /// This function returns the English greeting
        pub fn hello() -> String {"hello".to_string()}
        /// This function returns the English farewell
        pub fn goodbye() -> String {"goodbye".to_string()}
    }

    pub mod french
    {
        /// This function returns the French greeting
        pub fn hello() -> String {"bonjour".to_string()}
        /// This function returns the French farewell
        pub fn goodbye() -> String {"au revoir".to_string()}
    }

    pub mod german
    {
        /// This function returns the German greeting
        pub fn hello() -> String {"hallo".to_string()}
        /// This function returns the German farewell
        pub fn goodbye() -> String {"tschuess".to_string()}
    }
}

#[test]
fn english_greeting_test()
{
    assert_eq!("hello", greetings::english::hello());
    assert_eq!("goodbye", greetings::english::goodbye());
}

#[test]
fn french_greeting_test()
{
    assert_eq!("bonjour", greetings::french::hello());
    assert_eq!("au revoir", greetings::french::goodbye());
}