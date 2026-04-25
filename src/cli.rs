use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// ID of the target doujin
    #[arg(short, long)]
    pub doujin_ids: Option<Vec<u32>>,
}
