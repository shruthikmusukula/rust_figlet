use figlet_rs::FIGfont;
use structopt::StructOpt;

#[derive(StructOpt)]
struct CLI {
    content: String
}

fn main() {
    // Obtain CLI Arguments
    let args = CLI::from_args();

    // Set font for figlet program
    let standard_font = FIGfont::standand().unwrap();

    // Get text content from CLI args
    let figure = standard_font.convert(&args.content);
    assert!(figure.is_some());

    // Display figlet output
    println!("{}", figure.unwrap());
}
