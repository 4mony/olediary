use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Opt {
    /// the "dist" created bhy trunk diractory to be served for hydration.
    #[clap(short, long)]
    pub dir: PathBuf,
}