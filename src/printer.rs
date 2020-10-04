

pub struct Printer {
    
}

impl Printer {
    pub fn say(self: &Self, who: String) -> String {
      match who.as_str() {
        "" => "Hi!".to_owned(),
        _ => format!("Hi {}!", who).to_owned()
      }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_say_should_return_the_correct_say() {
        let printer = Printer {};
        let who = "Tom".to_owned();
        let actual = printer.say(who);
        let expected = "Hi Tom!".to_owned();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_say_should_return_without_space_on_empty_who() {
        let printer = Printer {};
        let who = "".to_owned();
        let actual = printer.say(who);
        let expected = "Hi!".to_owned();

        assert_eq!(actual, expected);
    }
}