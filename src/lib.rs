#[cfg(test)]
mod tests;

pub enum NearestOption {
    A,
    B,
}
pub mod charsets;
pub mod color;
pub mod ansi;

pub mod cell;

pub mod utils{
    use std::process::exit;



    pub fn expand_path(path: &str) -> String{
        match shellexpand::full(path) {
            Ok(s) => return s.to_string(),
            Err(e) => {
                eprint!("Fatal error while expanding path '{:?}' due to {:?}", path, e);
                exit(-1)
            },
        }
    }

    pub fn get_scaling(name: &str) -> image::imageops::FilterType{
        match name {
            "nearest" => image::imageops::FilterType::Nearest,
            "gaussian" => image::imageops::FilterType::Gaussian,
            _ => image::imageops::FilterType::Triangle
        }
    }
}