use std::collections::HashMap;

/// RPN Struct for holding the stack and variable hashmap
pub struct RPN {
    /// The main stack. Numbers and operators go here.
    pub stack: Vec<String>,
    /// A Hashmap used to hold temporary variables for advanced processing.
    pub vars: HashMap<String, String>,
}

impl RPN {
    /// Returns a instance of `RPN` with initialized values.
    ///
    /// # Example
    ///
    /// ```
    /// use rpn::RPN;
    /// let mut calc = RPN::new();
    /// ```
    pub fn new() -> Self {
        RPN {
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
    /// use rpn::RPN;
    ///
    /// let mut calc = RPN::new();
    ///
    /// calc.parse("5 2 + -3 - 10 +");
    ///
    /// let result = calc.peek();
    ///
    /// assert_eq!(result, "20")
    /// ```
    pub fn parse(&mut self, expression: &str) {
        let tokens: Vec<String> = expression
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        if tokens.len() == 0 {
            println!("Nothing to parse!")
        }

        for token in &tokens {
            match token.parse::<isize>() {
                Ok(value) => {
                    let last = tokens.last().unwrap().to_owned();
                    if last == value.to_string() {
                        panic!("Last item needs to be an operator!")
                    } else {
                        self.push(value.to_string())
                    }
                }
                Err(_) => match token.to_lowercase().as_str() {
                    "x" => self.exchange(),
                    "?" => self.stack_dump(),
                    "&" => self.var_dump(),
                    "+" => self.add(),
                    "-" => self.subtract(),
                    "*" => self.multiply(),
                    "/" => self.divide(),
                    "^" => self.exponent(),
                    _ => {
                        if token.chars().nth(0) == Some('!') {
                            let val = self.peek();
                            self.vars.insert(token.as_str()[1..].to_string(), val);
                        } else if token.chars().nth(0) == Some('@') {
                            let result = token.as_str()[1..].to_string();
                            if !result.is_empty() {
                                let entry = self.vars.get(&result).unwrap().to_owned();
                                self.push(entry)
                            } else {
                                self.push("".to_string())
                            }
                        } else {
                            panic!("Unknown operator or number: `{}`", token)
                        }
                    }
                },
            };
        }
    }

    ///
    /// Inserts a value at the top of `self.stack`.
    ///
    fn push(&mut self, value: String) {
        self.stack.push(value);
    }

    ///
    /// Removes the first entry from `self.stack` and returns it.
    ///
    /// Will panic if the stack is empty.
    ///
    fn pop(&mut self) -> String {
        self.stack.pop().expect("Stack is empty.")
    }

    ///
    /// Returns the value at the top of `self.stack` **without** removing it.
    ///
    /// Will panic if the stack is empty.
    ///
    pub fn peek(&mut self) -> String {
        self.stack.last().expect("Stack is empty.").to_string()
    }

    /// Adds the first two values on `self.stack` and
    /// pushes the sum to the top of `self.stack`.
    fn add(&mut self) {
        let x: isize = self.pop().parse().unwrap();
        let y: isize = self.pop().parse().unwrap();
        let result = x + y;
        self.push(result.to_string());
    }

    /// Subtracts the first two values on `self.stack` and
    /// pushes the difference to the top of `self.stack`.
    ///
    /// The equation here is `self.stack[1] - self.stack[0]` due the stack ordering.
    fn subtract(&mut self) {
        let x: isize = self.pop().parse().unwrap();
        let y: isize = self.pop().parse().unwrap();
        let result = y - x;
        self.push(result.to_string());
    }

    /// Multiplies the first two values on `self.stack` and
    /// pushes the result to the top of `self.stack`.
    fn multiply(&mut self) {
        let x: isize = self.pop().parse().unwrap();
        let y: isize = self.pop().parse().unwrap();
        let result = x * y;
        self.push(result.to_string());
    }

    /// Divides the first two values on `self.stack` and
    /// pushes the result to the top of `self.stack`.
    ///
    /// The equation here is `self.stack[1] / self.stack[0]` due the stack ordering.
    fn divide(&mut self) {
        let x: isize = self.pop().parse().unwrap();
        let y: isize = self.pop().parse().unwrap();
        let result = y / x;
        self.push(result.to_string());
    }

    /// Raises a base value to a specified power.
    ///
    /// The `base_val` is the first value off `self.stack`.
    /// The `power` is the second value off `self.stack`.
    fn exponent(&mut self) {
        let base_val: isize = self.pop().parse().unwrap();
        let power: u32 = self.pop().parse().unwrap();
        let result = base_val.pow(power);
        self.push(result.to_string());
    }

    /// Exchanges the position of the first two values on `self.stack`.
    ///
    /// If `self.stack` had `10, 2`, then `self.exchange()` would change this
    /// to `2, 10`
    ///
    /// Will panic if `self.stack` is empty.    
    fn exchange(&mut self) {
        let t = self.stack.pop().expect("Stack is empty.");
        let t1 = self.stack.pop().expect("Stack is empty.");
        self.stack.push(t);
        self.stack.push(t1);
    }

    /// Diagnostic function. Dumps the contents of `self.stack`.
    fn stack_dump(&mut self) {
        if self.stack.len() > 0 {
            print!("STACK:\n");
            for item in self.stack.clone() {
                println!("\tStack = {}", item);
            }
            print!("\n");
        }
    }

    /// Diagnostic function. Dumps the contents of `self.vars`.
    fn var_dump(&mut self) {
        if self.stack.len() > 0 {
            print!("TEMP VARS\n");
            for (key, value) in self.vars.clone() {
                println!("\tKey = {} = {}", key, value);
            }
            print!("\n");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::RPN;

    #[test]
    fn basic_notation() {
        let mut calc = RPN::new();
        calc.parse("5 2 + -3 - 10 +");
        let result = calc.peek();
        assert_eq!(result, "20")
    }

    #[test]
    fn exponent_notation() {
        let mut calc = RPN::new();
        calc.parse("5 5 ^ 125 - 30 /");
        let result = calc.peek();
        assert_eq!(result, "100")
    }

    #[test]
    fn manual_addition() {
        let mut calc = RPN::new();
        calc.push("10".to_string());
        assert_eq!(calc.peek(), "10");
        calc.push("99".to_string());
        assert_eq!(calc.peek(), "99");
        calc.add();
        assert_eq!(calc.peek(), "109")
    }

    #[test]
    fn manual_power_raising() {
        let mut calc = RPN::new();
        calc.push("5".to_string());
        calc.push("5".to_string());
        calc.exponent();
        assert_eq!(calc.peek(), "3125")
    }

    #[test]
    fn variable_testing() {
        let mut calc = RPN::new();
        calc.parse("50 20 + !temp");
        calc.pop();
        calc.parse("2 @temp *");
        assert_eq!(calc.peek(), "140")
    }
}
