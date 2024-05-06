use std::{
    fs::{self, File},
    io::Write,
    process::ExitCode,
};

use argh::FromArgs;
use clir_rs::{
    cell::{self, CellGrid, CELL_H, CELL_W},
    charsets,
    color::Color,
    utils,
};
use image::{io::Reader as ImageReader, DynamicImage};

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
    /// When not available or disabled, autosize sets width to 100, height is derived from aspect ratio . If --no-keep-aspect is set, height will be set to 25
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

    /// when set, doesn't print out the resulting unicode art. Still prints debug & other information
    #[argh(switch)]
    no_print: bool,

    /// specifies the character set to use. Valid options are ["braille", "classic"]. Defaults to classic not available [default: "classic"]
    #[argh(option)]
    charset: Option<String>,
}

const DEFAULT_WIDTH: usize = 100;
const DEFAULT_HEIGHT: usize = 25;

enum RenderMode {
    Color,
    NoColor,
    PlainText,
}

struct RenderSettings {
    im_height: u32,
    im_width: u32,
    render_mode: RenderMode,
    output: Option<String>,
    src: DynamicImage,
}

#[derive(Debug)]
enum RenderSettingsFromArgsErrs {
    DecodeError(image::ImageError),
    IoError(std::io::Error),
}

impl RenderSettings {
    // Reduces width o height to match the aspect ratio
    pub fn keep_aspect(width: usize, height: usize, aspect: f32) -> (usize, usize) {
        let width_ = CELL_W * width;
        let height_ = CELL_H * height;
        let new_width = (height_ as f32 * aspect / CELL_W as f32).floor();
        let new_height = (width_ as f32 / aspect / CELL_H as f32).floor();
        if new_height < (height as f32) {
            (width as usize, new_height as usize)
        } else {
            (new_width as usize, height)
        }
    }

    pub fn autodetected_size() -> (usize, usize) {
        match termsize::get() {
            Some(size) => (size.cols as usize, size.rows as usize),
            None => (DEFAULT_WIDTH, DEFAULT_HEIGHT),
        }
    }

    pub fn from_args(args: &CliArgs) -> Result<Self, RenderSettingsFromArgsErrs> {
        if args.debug {
            println!("Expanding source image '{:?}'", &args.source)
        }
        let expanded = utils::expand_path(&args.source);
        if args.debug {
            println!("Reading image from '{:?}'", expanded)
        }

        let img = match ImageReader::open(expanded) {
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
            let (dw, dh) = if args.no_autosize {
                (DEFAULT_WIDTH, DEFAULT_HEIGHT)
            } else {
                Self::autodetected_size()
            };
            let suggested_size = if args.no_keep_aspect {
                (dw, dh)
            } else {
                Self::keep_aspect(dw, dh, aspect)
            };
            (
                args.width.unwrap_or(suggested_size.0) * cell::CELL_W,
                args.height.unwrap_or(suggested_size.1) * cell::CELL_H,
            )
        };

        Ok(Self {
            im_height: output_size.1 as u32,
            im_width: output_size.0 as u32,
            render_mode: if args.plain_text {
                RenderMode::PlainText
            } else if args.no_color {
                RenderMode::NoColor
            } else {
                RenderMode::Color
            },
            output: args.output.clone(),
            src: img,
        })
    }
}

fn main() -> ExitCode {
    let before_cmd = Instant::now();
    let args: CliArgs = argh::from_env();

    if args.debug {
        println!("Runnning with arguments: {:#?}", args);
    }
    let config = match RenderSettings::from_args(&args) {
        Ok(x) => x,
        Err(err) => {
            eprintln!("Fatal error: {:#?}", err);
            return ExitCode::FAILURE;
        }
    };

    if args.debug {
        println!(
            "Resizing source image to {}x{}...",
            config.im_width, config.im_height
        );
    }
    let img = config.src.resize_exact(
        config.im_width,
        config.im_height,
        image::imageops::FilterType::Triangle,
    );

    use std::time::Instant;

    let before_cell = Instant::now();
    let mut cells = CellGrid::from(&img.clone().into());
    let cell_time = before_cell.elapsed();

    let before_round = Instant::now();
    let mut colored = false;
    let computed = match config.render_mode {
        RenderMode::Color => {
            colored = true;
            cells.to_computed()
        }
        RenderMode::NoColor => cells.to_computed(),
        RenderMode::PlainText => cells.to_computed_ab(&Color::WHITE, &Color::TRANSPARENT),
    };
    let round_cell_time = before_round.elapsed();

    let before_string = Instant::now();
    let (s, _) = computed.to_string(
        colored,
        Some(charsets::get_charset(
            &args.charset.unwrap_or("".to_string()),
        )),
    );
    let string_time = before_string.elapsed();

    if args.debug {
        match fs::create_dir("./clir_rs_debug/") {
            Ok(_) => {}
            Err(e) => eprintln!("Fatal err, failed to create debug output dir {:?}", e),
        };
        cells.save_as("./clir_rs_debug/colored_cells.png").unwrap();
        cell::round_cells_with_ab(&mut cells.cells, &Color::WHITE, &Color::TRANSPARENT);
        cells.save_as("./clir_rs_debug/bw_cells.png").unwrap();
    }
    if !args.no_print {
        println!("{}", s);
    }

    if args.debug {
        println!(
            "Source Image Size ({}x{}={}) | Final Image size ({}x{}={}) | Cells count: {} ({}x{}={})",
            config.src.width(),
            config.src.height(),
            config.src.width() * config.src.height(),
            img.width(),
            img.height(),
            img.width() * img.height(),
            cells.len(),
            cells.width(),
            cells.height(),
            cells.width() * cells.height()
        );
        println!(
            "Cell Generate Time: {:.2?} | Round Cell Pixels time: {:.2?} | String time: {:.2?} | Total compute time {:.2?}",
            cell_time, round_cell_time, string_time, round_cell_time + string_time + cell_time
        );
    }

    match args.output {
        Some(path) => {
            let expanded = utils::expand_path(&path);
            match File::create(&expanded) {
                Ok(mut file) => match file.write_all(s.as_bytes()) {
                    Ok(_) => {
                        if args.debug {
                            println!("Wrote output to {:?}", utils::expand_path(&expanded))
                        }
                    }

                    Err(e) => eprintln!(
                        "Failed to write output to path at '{:?}' due to {:?}",
                        path, e
                    ),
                },
                Err(e) => eprintln!(
                    "Failed to write output to path at '{:?}' due to {:?}",
                    path, e
                ),
            }
        }
        None => (),
    }

    print!("Command completed in: {:.2?}", before_cmd.elapsed());
    ExitCode::SUCCESS
}
