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

    /// disables automatic resizing of output size to fit the terminal if available. Using --width or --height will override the detected values.
    /// 
    /// When not available or disabled, autosize sets width to 100, height is derived from aspect ratio . If --no_keep_aspect is set, height will be set to 100
    #[argh(switch)]
    no_autosize: bool,

    /// specify width of the output.
    #[argh(option, short='w')]
    width: Option<usize>,

    /// specify height of the output.
    #[argh(option, short='h')]
    height: Option<usize>,

    /// disables keeping of aspect ratio when resizing images. No effect when both --width & --height is used.
    #[argh(switch)]
    no_keep_aspect: bool,

    /// disables colors, in rendered output. Result will be black & white
    #[argh(switch)]
    no_color:bool,

    /// enables plain text mode, useful for rendering unicode art.
    #[argh(switch)]
    plain_text:bool,

    /// enable debug outputs, which will be stored in `./clir_rs_debug/`
    #[argh(switch)]
    debug: bool,
    
}

const DEFAULT_WIDTH: usize = 100;
const DEFAULT_HEIGHT: usize = 100;

fn main() {
    let args: Args = argh::from_env();

    println!("{:#?}",args)
}