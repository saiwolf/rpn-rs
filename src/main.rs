// # RPN Calculator
//
// This is a small program that parses a Reverse Polish Notation Equation
// and returns the result.
//
// This program is based off https://gist.github.com/wd5gnr/68d067c3c42a2e0e9a27b083e01f7080#file-rpn-py
// by https://github.com/wd5gnr

use anyhow::Result;
use clap::Parser;
use rpn_calculator::RPNParser;

#[derive(Parser)]
#[command(name = "Reverse Polish Notation (RPN) Calculator",
          version, about, long_about = None)]
struct Cli {
    #[arg(
        short,
        long,
        help = "Reverse Polish Notation Equation",
        conflicts_with = "test_info"
    )]
    expression: Option<String>,
    #[arg(
        short,
        long,
        help = "Show some test info and exit.",
        conflicts_with = "expression"
    )]
    test_info: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    if args.test_info {
        dump_test_info()?
    }
    if let Some(v) = args.expression {
        let mut calc = RPNParser::new();
        calc.parse(&v)?;
        let result = calc.peek()?;
        println!("{}", result)
    }
    Ok(())
}

fn dump_test_info() -> Result<()> {
    let mut calc = RPNParser::new();
    println!("\t===STACK DUMP===\n");
    println!("Equation: 10 + 20");
    println!("Expression: 10 20 +");
    calc.push("10".to_string());
    calc.push("20".to_string());
    calc.push("+".to_string());
    println!("Dumping stack:\n");
    calc.stack_dump();
    println!("Clearing parser memory...\n");
    calc.clear();
    println!("\t===VAR DUMP===\n");
    println!("Equation: !temp = 10 + 20");
    println!("Expression: 50 20 + !temp");
    calc.parse("50 20 + !temp")?;
    calc.var_dump();
    std::process::exit(0)
}
