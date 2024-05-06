use argh::FromArgs;

#[derive(FromArgs, Debug)]
/// Renders an image to the console as unicode art
struct Args {
    /// path to the source image
    #[argh(positional)]
    source: String,
    
    /// saves output to this path.
    #[argh(option)]
    output: Option<String>,

    /// disables automatic resizing of output size to fit the terminal if available. Using --width or --height overrides the detected values.
    #[argh(switch)]
    no_autosize: bool,

    /// width of the output
    #[argh(option, short='w')]
    width: Option<usize>,

    /// height of the output
    #[argh(option, short='h')]
    height: Option<usize>,

    /// disables keeping of aspect ratio when resizing images. No effect when both --width & --height is used.
    #[argh(switch)]
    no_keep_aspect: bool,

    /// enable debug outputs, which will be stored in `./clir_rs_debug/`
    #[argh(switch)]
    debug: bool,
    
}

fn main() {
    let args: Args = argh::from_env();

    println!("{:#?}",args)
}