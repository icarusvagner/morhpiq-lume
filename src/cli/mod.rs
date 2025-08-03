use clap::Parser;

use crate::MORPHIQ_LOWERCASE;

#[derive(Parser, Debug)]
#[command(
    name = MORPHIQ_LOWERCASE,
    bin_name = MORPHIQ_LOWERCASE
)]
struct Args;
