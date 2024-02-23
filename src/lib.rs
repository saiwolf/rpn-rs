use anyhow::{Context, Result};
use std::collections::HashMap;

/// Parser Struct for holding the stack array and variable hashmap
pub struct Parser {
    /// The main stack. Numbers and operators go here.
    pub stack: Vec<String>,
    /// A Hashmap used to hold temporary variables for advanced processing.
    pub vars: HashMap<String, String>,
}

impl Parser {
    /// Returns a instance of `Parser` with initialized values.
    ///
    /// # Example
    ///
    /// ```
    /// use rpn_calculator::Parser;
    /// let mut calc = Parser::new();
    /// ```
    pub fn new() -> Self {
        Parser {
            stack: Vec::new(),
            vars: HashMap::new(),
        }
    }

    /// Parses a Reverse Polish Notation Equation and calculates the result.
    /// # Arguments
    ///
    /// * `expression` - A string slice that holds the equation to calculate.
    ///
    /// # Example
    ///
    /// ```
    /// use rpn_calculator::Parser;
    ///
    /// let mut calc = Parser::new();
    ///
    /// calc.parse("5 2 + -3 - 10 +").unwrap(); // .parse() returns a Result
    ///
    /// let result = calc.peek().unwrap(); // .peek() returns a Result
    ///
    /// assert_eq!(result, "20")
    /// ```
    pub fn parse(&mut self, expression: &str) -> Result<()> {
        let tokens: Vec<String> = expression
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        if tokens.len() == 0 {
            eprintln!("Nothing to parse!")
        }

        for token in &tokens {
            match token.parse::<isize>() {
                Ok(value) => {
                    let last = tokens.last().unwrap().to_owned();
                    if last == value.to_string() {
                        eprintln!("Last item needs to be an operator!")
                    } else {
                        self.push(value.to_string())?
                    }
                }
                Err(_) => match token.to_lowercase().as_str() {
                    "x" => self.exchange()?,
                    "?" => self.stack_dump(),
                    "&" => self.var_dump(),
                    "+" => self.add()?,
                    "-" => self.subtract()?,
                    "*" => self.multiply()?,
                    "/" => self.divide()?,
                    "^" => self.exponent()?,
                    _ => {
                        if token.chars().nth(0) == Some('!') {
                            let val = self.peek().unwrap();
                            self.vars.insert(token.as_str()[1..].to_string(), val);
                        } else if token.chars().nth(0) == Some('@') {
                            let result = token.as_str()[1..].to_string();
                            if !result.is_empty() {
                                let entry = self.vars.get(&result).unwrap().to_owned();
                                self.push(entry)?
                            } else {
                                self.push("".to_string())?
                            }
                        } else {
                            panic!("Unknown operator or number: `{}`", token)
                        }
                    }
                },
            };
        }
        Ok(())
    }

    ///
    /// Inserts a value at the top of `self.stack`.
    ///
    pub fn push(&mut self, value: String) -> Result<()> {
        Ok(self.stack.push(value))
    }

    ///
    /// Removes the first entry from `self.stack` and returns it.
    ///
    /// Will panic if the stack is empty.
    ///
    pub fn pop(&mut self) -> Result<String> {
        let result = self.stack.pop().context("Stack is empty!")?;
        Ok(result)
    }

    ///
    /// Returns the value at the top of `self.stack` **without** removing it.
    ///
    /// Will panic if the stack is empty.
    ///
    pub fn peek(&mut self) -> Result<String> {
        let result = self.stack.last().context("Stack is empty!")?.to_string();
        Ok(result)
    }

    /// Clears the parser memory.
    pub fn clear(&mut self) {
        self.stack = Vec::new();
        self.vars = HashMap::new();
    }

    /// Adds the first two values on `self.stack` and
    /// pushes the sum to the top of `self.stack`.
    pub fn add(&mut self) -> Result<()> {
        let x: isize = self.pop()?.parse()?;
        let y: isize = self.pop()?.parse()?;
        let result = x + y;
        self.push(result.to_string())?;
        Ok(())
    }

    /// Subtracts the first two values on `self.stack` and
    /// pushes the difference to the top of `self.stack`.
    ///
    /// The equation here is `self.stack[1] - self.stack[0]` due the stack ordering.
    pub fn subtract(&mut self) -> Result<()> {
        let x: isize = self.pop()?.parse()?;
        let y: isize = self.pop()?.parse()?;
        let result = y - x;
        Ok(self.push(result.to_string())?)
    }

    /// Multiplies the first two values on `self.stack` and
    /// pushes the result to the top of `self.stack`.
    pub fn multiply(&mut self) -> Result<()> {
        let x: isize = self.pop()?.parse()?;
        let y: isize = self.pop()?.parse()?;
        let result = x * y;
        Ok(self.push(result.to_string())?)
    }

    /// Divides the first two values on `self.stack` and
    /// pushes the result to the top of `self.stack`.
    ///
    /// The equation here is `self.stack[1] / self.stack[0]` due the stack ordering.
    pub fn divide(&mut self) -> Result<()> {
        let x: isize = self.pop()?.parse()?;
        let y: isize = self.pop()?.parse()?;
        let result = y / x;
        Ok(self.push(result.to_string())?)
    }

    /// Raises a base value to a specified power.
    ///
    /// The `base_val` is the first value off `self.stack`.
    /// The `power` is the second value off `self.stack`.
    pub fn exponent(&mut self) -> Result<()> {
        let base_val: isize = self.pop()?.parse()?;
        let power: u32 = self.pop()?.parse()?;
        let result = base_val.pow(power);
        Ok(self.push(result.to_string())?)
    }

    /// Exchanges the position of the first two values on `self.stack`.
    ///
    /// If `self.stack` had `10, 2`, then `self.exchange()` would change this
    /// to `2, 10`
    ///
    /// Will panic if `self.stack` is empty.    
    pub fn exchange(&mut self) -> Result<()> {
        let t = self.pop()?;
        let t1 = self.pop()?;
        self.push(t)?;
        self.push(t1)?;
        Ok(())
    }

    /// Diagnostic function. Dumps the contents of `self.stack`.    
    pub fn stack_dump(&self) {
        if self.stack.len() > 0 {
            print!("STACK:\n");
            for item in self.stack.to_owned() {
                println!("\tStack = {}", item);
            }
            print!("\n");
        }
    }

    /// Diagnostic function. Dumps the contents of `self.vars`.
    pub fn var_dump(&self) {
        if self.stack.len() > 0 {
            print!("TEMP VARS\n");
            for (key, value) in self.vars.to_owned() {
                println!("\tKey = {} = {}", key, value);
            }
            print!("\n");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_notation() {
        let mut calc = Parser::new();
        calc.parse("5 2 + -3 - 10 +").unwrap();
        let result = calc.peek().unwrap();
        assert_eq!(result, "20")
    }

    #[test]
    fn exponent_notation() {
        let mut calc = Parser::new();
        calc.parse("5 5 ^ 125 - 30 /").unwrap();
        let result = calc.peek().unwrap();
        assert_eq!(result, "100")
    }

    #[test]
    fn manual_addition() {
        let mut calc = Parser::new();
        calc.push("10".to_string()).unwrap();
        assert_eq!(calc.peek().unwrap(), "10");
        calc.push("99".to_string()).unwrap();
        assert_eq!(calc.peek().unwrap(), "99");
        calc.add().unwrap();
        assert_eq!(calc.peek().unwrap(), "109")
    }

    #[test]
    fn manual_power_raising() {
        let mut calc = Parser::new();
        calc.push("5".to_string()).unwrap();
        calc.push("5".to_string()).unwrap();
        calc.exponent().unwrap();
        assert_eq!(calc.peek().unwrap(), "3125")
    }

    #[test]
    fn variable_testing() {
        let mut calc = Parser::new();
        calc.parse("50 20 + !temp").unwrap();
        calc.pop().unwrap();
        calc.parse("2 @temp *").unwrap();
        assert_eq!(calc.peek().unwrap(), "140")
    }
}
