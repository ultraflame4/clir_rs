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
    outputs::AsciiImageRenderer,
    utils,
};
use image::{io::Reader as ImageReader, DynamicImage};
use is_url::is_url;

#[derive(FromArgs, Debug)]
/// Renders an image to the console as unicode art
struct CliArgs {
    /// filepath or url to the source image. Note that not all urls might work, it is recommended to download the image separately as an png first.
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

    /// specifies the character set to use. Valid options are ["braille", "classic"]. Uses default for unknown values [default: "classic"]
    #[argh(option)]
    charset: Option<String>,

    /// sets the method use to scale the image. Valid options are ["nearest","linear","gaussian"]. Uses default for unknown values  [default: "linear"]
    #[argh(option)]
    scaling: Option<String>,

    /// sets the threshold for transparency. When alpha < transparency_t, it resets the back or fore color for the character. If both fore & back is transparent, it replaces it with a space.
    /// This effect can only be seen in terminals where the background is not black. [default: 0.9]
    #[argh(option, short = 't')]
    transparency_t: Option<f32>,

    /// inverts the fore and background cell mask. Colors are also inverted (such that there is no effect on color) respectively.
    #[argh(switch)]
    invert_cell: bool,
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

impl RenderSettings {
    // Reduces width or height to match the aspect ratio
    pub fn keep_aspect(
        width: usize,
        height: usize,
        aspect: f32,
        use_width: Option<bool>,
    ) -> (usize, usize) {
        let width_ = CELL_W * width;
        let height_ = CELL_H * height;
        let new_width = (height_ as f32 * aspect / CELL_W as f32).floor();
        let new_height = (width_ as f32 / aspect / CELL_H as f32).floor();
        if use_width.unwrap_or(new_height < (height as f32)) {
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

    pub fn from_args(args: &CliArgs, img: DynamicImage) -> Self {
        let aspect = img.width() as f32 / img.height() as f32;

        let output_size = if args.use_original_image_size {
            (img.width() as usize, img.height() as usize)
        } else {
            let (dw, dh) = if args.no_autosize {
                (DEFAULT_WIDTH, DEFAULT_HEIGHT)
            } else {
                Self::autodetected_size()
            };

            let unwrapped_size = (args.width.unwrap_or(dw), args.height.unwrap_or(dh));

            let (fw, fh) = if args.no_keep_aspect || (args.width.is_some() && args.height.is_some())
            {
                unwrapped_size
            } else {
                // When use_width is true, it will always scale the height instead.
                // Hence by using width.is_some(), when -w, it will scale height,
                // when -h, it will scale width. -w & -h should never happen because it is checked in the previous if condition
                Self::keep_aspect(
                    unwrapped_size.0,
                    unwrapped_size.1,
                    aspect,
                    Some(args.width.is_some()),
                )
            };

            (fw * CELL_W, fh * CELL_H)
        };

        Self {
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
        }
    }
}

fn read_image(path: &str, debug: bool) -> anyhow::Result<DynamicImage> {
    anyhow::Ok(if is_url(&path) {
        if debug {
            println!("Requesting image from url '{:?}'", path);
        }
        let res = reqwest::blocking::get(path)?;
        if debug {
            println!("Response '{:#?}'", res);
        }
        let bytes = res.bytes()?;
        image::load_from_memory(&bytes)?
    } else {
        if debug {
            println!("Expanding source image '{:?}'", &path);
        }
        let expanded = utils::expand_path(&path);
        println!("Reading image from '{:?}'", expanded);
        ImageReader::open(expanded)?.decode()?
    })
}

fn main() -> ExitCode {
    let before_cmd = Instant::now();
    let args: CliArgs = argh::from_env();

    if args.debug {
        println!("Running with arguments: {:#?}", args);
    }

    let img = match read_image(&args.source, args.debug) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Fatal error while trying to read image: {:#?}", e);
            return ExitCode::FAILURE;
        }
    };

    let config = RenderSettings::from_args(&args, img);

    if args.debug {
        println!(
            "Resizing source image to {}x{}...",
            config.im_width, config.im_height
        );
    }
    let img = config.src.resize_exact(
        config.im_width,
        config.im_height,
        utils::get_scaling(&args.scaling.unwrap_or("".to_owned())),
    );

    use std::time::Instant;

    let (mut cells, cell_time) = {
        let now = Instant::now();
        let cells = CellGrid::from(&img.clone().into());
        (cells, now.elapsed())
    };

    let mut colored = false;
    
    let (computed, compute_time) = {
        let now = Instant::now();
        let computed = match config.render_mode {
            RenderMode::Color => {
                colored = true;
                cells.compute(args.invert_cell)
            }
            RenderMode::NoColor => cells.compute(args.invert_cell),
            RenderMode::PlainText => {
                cells.compute_ab(&Color::WHITE, &Color::BLACK, args.invert_cell)
            }
        };
        (computed, now.elapsed())
    };

    let (s, string_time) = {
        let now = Instant::now();
        let (img, _) = AsciiImageRenderer::render(
            &computed,
            colored,
            Some(charsets::get_charset(
                &args.charset.unwrap_or("".to_string()),
            )),
            args.transparency_t.unwrap_or(0.9),
        );
        (img, now.elapsed())
    };

    if args.debug {
        match fs::create_dir("./clir_rs_debug/") {
            Ok(_) => {}
            Err(e) => eprintln!("Warning: failed to create debug output dir {:?}", e),
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
            cell_time, compute_time, string_time, compute_time + string_time + cell_time
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
