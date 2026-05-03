use clap::{Args, Parser, ValueEnum};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum SearchSort {
    Date,
    PopularAllTime,
    PopularToday,
    PopularWeek,
    PopularMonth,
}

#[derive(Args, Debug)]
#[group(id = "search", multiple = true)]
pub struct Search {
    #[arg(short, long, group = "search", value_name = "TAGS")]
    pub tags: Option<Vec<String>>,
    #[arg(short, long, group = "search", value_name = "DIGITS")]
    pub alike: Option<u32>,
    #[arg(short, long, group = "search", value_name = "FILTER", value_enum, default_value_t = SearchSort::PopularAllTime)]
    pub sort: SearchSort,
    // #[arg(short, long, value_parser = clap::value_parser!(u16).range(1..20), default_value_t = 10)]
    // pub limit: u16,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    #[arg(short, long, value_name = "DIGITS", conflicts_with = "search")]
    pub digits: Option<Vec<u32>>,

    #[command(flatten)]
    pub search: Search,
}
