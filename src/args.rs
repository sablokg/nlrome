use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "nlrome",
    version = "1.0",
    about = "nlr from identification to deep learning.
       ************************************************
       Author Gaurav Sablok,
       Email: gsablok@proton.me
      ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// NLRResistanceMiner
    Miner {
        /// id for the resistance gene
        idstring: String,
    },
    /// NLRResistanceFetcher
    Fetcher {
        /// id for the resistance gene
        idstring: String,
        /// dnasequence or protein sequence
        sequence: String,
    },
    /// train NLR
    TrainNLR {
        /// path to the fasta file
        fastafile: String,
        /// path to the label file
        label: String,
        /// epochs number
        epochs: String,
    },
}
