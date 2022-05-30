use orgarq::assemble;

fn main() {
    match assemble("ex.qck", "ex.fita") {
        Ok(_) => (),
        Err(why) => println!("{}", why),
    }
}
