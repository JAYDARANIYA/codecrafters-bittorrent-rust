use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub subcmd: Commands,
}

#[derive(Subcommand)]
#[clap(rename_all = "snake_case")]
pub enum Commands {
    Decode {
        encoded_value: String,
    },
    Info {
        path: String,
    },
    Peers {
        path: String,
    },
    Handshake {
        path: String,
        peer: String,
    },
    DownloadPiece {
        #[arg(short, long, value_name = "FILE-PATH")]
        out: String,
        path: String,
        piece_index: u32,
    },
    Download {
        #[arg(short, long, value_name = "FILE-PATH")]
        out: String,
        path: String,
    },
}
