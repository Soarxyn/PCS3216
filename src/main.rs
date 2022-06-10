use sisprog::processor::assembler::assemble;

fn main() {
    match assemble("ex.qck", "ex.fita") {
        Ok(_) => (),
        Err(why) => println!("{}", why),
    }
}
