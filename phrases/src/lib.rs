pub mod greetings
{
    pub mod english
    {
        pub fn hello() -> String {"hello".to_string()}
        pub fn goodbye() -> String {"goodbye".to_string()}
    }

    pub mod french
    {
        pub fn hello() -> String {"bonjour".to_string()}
        pub fn goodbye() -> String {"au revoir".to_string()}
    }

    pub mod german
    {
        pub fn hello() -> String {"hallo".to_string()}
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