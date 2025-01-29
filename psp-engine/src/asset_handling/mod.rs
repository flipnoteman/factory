use psp::dprintln;

pub mod asset_handler;
pub mod assets;

#[macro_export]
macro_rules! add_asset {
    ($path:literal) => {
        dprintln!("{}", $path);
    };
}