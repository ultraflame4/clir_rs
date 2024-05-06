use std::process::{exit, ExitCode};

use argh::FromArgs;
use image::io::Reader as ImageReader;

#[derive(FromArgs, Debug)]
/// Renders an image to the console as unicode art
struct CliArgs {
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

    /// specify width of the output in number of chars.
    #[argh(option, short = 'w')]
    width: Option<usize>,

    /// specify height of the output in number of rows.
    #[argh(option, short = 'h')]
    height: Option<usize>,

    /// disables keeping of aspect ratio when resizing images. No effect when both --width & --height is used.
    #[argh(switch)]
    no_keep_aspect: bool,

    /// disables colors, in rendered output. Result will be black & white
    #[argh(switch)]
    no_color: bool,

    /// enables plain text mode, useful for rendering unicode art.
    #[argh(switch)]
    plain_text: bool,

    /// enable debug outputs, which will be stored in `./clir_rs_debug/`
    #[argh(switch)]
    debug: bool,

    /// overrides all size options. Uses the orginal image's size. Calculation is (image.width / CELL_W, image.height / CELL_H) Where CELL_W & CELL_H is typically 2 & 4 respectively.
    #[argh(switch)]
    use_original_image_size: bool,
}

const DEFAULT_WIDTH: usize = 100;
const DEFAULT_HEIGHT: usize = 100;

enum RenderMode {
    Color,
    NoColor,
    PlainText,
}

struct RenderSettings {
    height: u32,
    width: u32,
    render_mode: RenderMode,
    output: Option<String>,
}

#[derive(Debug)]
enum RenderSettingsFromArgsErrs {
    DecodeError(image::ImageError),
    IoError(std::io::Error),
}

impl RenderSettings {
    pub fn scale_aspect(width: usize, aspect: f32) -> usize {
        let rounded: u32 = (width as f32 / aspect).round() as u32;
        rounded as usize
    }

    pub fn autodetected_size(aspect: f32) -> (usize, usize) {
        match termsize::get() {
            Some(size) => (size.cols as usize, size.rows as usize),
            None => (DEFAULT_WIDTH, Self::scale_aspect(DEFAULT_WIDTH, aspect)),
        }
    }

    pub fn from_args(args: &CliArgs) -> Result<Self, RenderSettingsFromArgsErrs> {
        let img = match ImageReader::open(&args.source) {
            Ok(img_data) => match img_data.decode() {
                Ok(x) => x,
                Err(e) => return Err(RenderSettingsFromArgsErrs::DecodeError(e)),
            },
            Err(e) => return Err(RenderSettingsFromArgsErrs::IoError(e)),
        };

        let aspect = img.width() as f32 / img.height() as f32;

        let output_size = if args.use_original_image_size {
            (img.width() as usize, img.height() as usize)
        } else {
            let suggested_size = Self::autodetected_size(aspect);
            (
                args.width.unwrap_or(suggested_size.0),
                args.height.unwrap_or(suggested_size.1),
            )
        };

        Ok(Self {
            height: output_size.1 as u32,
            width: output_size.0 as u32,
            render_mode: if args.plain_text {
                RenderMode::PlainText
            } else if args.no_color {
                RenderMode::NoColor
            } else {
                RenderMode::Color
            },
            output: args.output.clone(),
        })
    }
}

fn main() -> ExitCode {
    let args: CliArgs = argh::from_env();

    if args.debug{
        println!("Runnning with arguments: {:#?}", args);
    }
    let settings = match RenderSettings::from_args(&args) {
        Ok(x) => x,
        Err(err) => {
            eprintln!("Fatal error: {:#?}", err);
            return ExitCode::FAILURE;
        },
    };

    ExitCode::SUCCESS
}
