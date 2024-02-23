use anyhow::Result;
use clap::Parser;
use rpn_calculator::RPNParser;

fn main() -> Result<()> {
    Ok(dump_test_info()?)
}

fn dump_test_info() -> Result<()> {
    let mut calc = RPNParser::new();
    println!("Dumping stack:\n");
    calc.push("10".to_string())?;
    calc.push("20".to_string())?;
    calc.push("+".to_string())?;
    calc.stack_dump();
    println!("Clearing parser memory...\n");
    calc.clear();
    println!("Dumping temporary variables...\n");
    calc.parse("50 20 + !temp")?;
    calc.var_dump();
    Ok(())
}
