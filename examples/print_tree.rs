use std::fs::File;
use flate2::read::{GzDecoder};
use serde_nbt::nbt::parser::{Parser, ValueType};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let filename = &args[1];
    println!("opening {}", filename);

    let file = File::open(filename)?;
    let decoder = GzDecoder::new(file);

    let mut parser = Parser::new(decoder);
    parser.next()?;
    print_tag(&mut parser, 0)?;

    Ok(())
}

fn print_tag<R>(parser: &mut Parser<R>, indent: usize) -> Result<()> where R: std::io::Read  {
    print_indent(indent);

    print!("\"{}\": ", parser.get_string_value()?);

    parser.next()?;
    print_value(parser, indent)?;

    print!("\n");

    Ok(())
}

fn print_value<R>(parser: &mut Parser<R>, indent: usize) -> Result<()> where R: std::io::Read  {
    match parser.get_value_type() {
        ValueType::I8 => print!("{}b", parser.get_i8_value()?),
        ValueType::I16 => print!("{}s", parser.get_i16_value()?),
        ValueType::I32 => print!("{}", parser.get_i32_value()?),
        ValueType::I64 => print!("{}l", parser.get_i64_value()?),
        ValueType::F32 => print!("{}f", parser.get_f32_value()?),
        ValueType::F64 => print!("{}d", parser.get_f64_value()?),
        ValueType::String => print!("\"{}\"", parser.get_string_value()?),
        ValueType::MapBegin => print_map(parser, indent)?,
        ValueType::SeqBegin => print_seq(parser, indent)?,
        _ => panic!(),
    }
    Ok(())
}

fn print_seq<R>(parser: &mut Parser<R>, indent: usize) -> Result<()> where R: std::io::Read {
    print!("[");

    loop {
        parser.next()?;
        let value_type = parser.get_value_type();
        if value_type == ValueType::SeqEnd {
            break;
        }

        print!("\n");
        print_indent(indent + 1);
        print_value(parser, indent + 1)?;
        print!(",");
    }

    print!("\n");
    print_indent(indent);
    print!("]");

    Ok(())
}

fn print_map<R>(parser: &mut Parser<R>, indent: usize) -> Result<()> where R: std::io::Read {
    print!("{{\n");

    loop {
        parser.next()?;
        let value_type = parser.get_value_type();
        if value_type == ValueType::MapEnd {
            break;
        }

        print_tag(parser, indent + 1)?;
    }

    print_indent(indent);
    print!("}}");

    Ok(())
}

fn print_indent(indent: usize) {
    for _i in 0..indent {
        print!("  ");
    }
}

