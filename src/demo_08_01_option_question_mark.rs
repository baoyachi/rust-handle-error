#[derive(Debug)]
enum Error {
    OptionError(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Error::OptionError(ref e) => e.fmt(f),
        }
    }
}

pub type Result<I> = std::result::Result<I, Error>;

fn foo(index: i32) -> Option<String> {
    if index > 60 {
        return Some("bar".to_string());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() -> Result<()> {
        //painc eror:
        //cause:the trait `std::convert::From<std::option::NoneError>` is not implemented for `demo_08_option_question_mark::Error`
//        let bar = foo(60)?;
//        assert_eq!("bar", bar);
        Ok(())
    }
}

//fn main() -> Result<()> {
//    let bar = foo(60)?;
//    assert_eq!("bar", bar);
//    Ok(())
//}