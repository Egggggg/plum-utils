use std::{env, fs::File, io::Write};

use tera::{Context, Tera};

use plum::{error::ChumskyAriadne, interpreter::interpret, value::Value};

fn main() {
    let args = &env::args().collect::<Vec<String>>();
    let input = args[1].clone();
    let template = args[2].clone();
    let output = args[3].clone();

    let file = std::fs::read(&input).unwrap();
    let template = std::fs::read(&template).unwrap();

    let source = String::from_utf8(file).unwrap();
    let template = String::from_utf8(template).unwrap();

    let evaluated = interpret(&source);

    let vars = match evaluated {
        Err(errs) => {
            for err in errs {
                err.display(&input, &source, 0);
            }

            return;
        }
        Ok(out) => out.values,
    };

    let mut ctx = Context::new();
    ctx.insert("vars", &vars);

    let tera = Tera::one_off(&template, &ctx, true).unwrap();

    let mut f = File::create(&output).expect(&format!("Couldn't create output file {output}"));

    f.write_all(tera.as_bytes())
        .expect(&format!("Couldn't write to output file {output}"));

    println!("Successfully wrote to {output}");
}
