extern crate lalrpop;

fn main() {
    // lalrpop::process_root().unwrap();
    lalrpop::Configuration::new()
        .use_colors_if_tty()
        .process_file("./src/grammar.lalrpop")
        .unwrap();
}
