mod args;
use crate::args::CommandParse;
use crate::args::Commands;
use clap::Parser;
mod locatenlr;
mod resistancefetcher;
mod resistanceminer;
use crate::resistancefetcher::prgdb_sequence_fetcher;
use crate::resistanceminer::mine_resistance_genes;
mod axonconvd;
use crate::axonconvd::trainaxonml;

/*
Gaurav Sablok
gsablok@proton.me
 */

fn main() {
    let argparse = CommandParse::parse();
    match &argparse.command {
        Commands::Fetcher { idstring, sequence } => {
            let command = prgdb_sequence_fetcher(idstring, sequence).unwrap();
            println!("The command has finished:{}", command);
        }
        Commands::Miner { idstring } => {
            let genbank_id = mine_resistance_genes(idstring).unwrap();
            println!("Resistance gene GenBank ID: {}", genbank_id);
        }
        Commands::TrainNLR {
            fastafile,
            label,
            epochs,
        } => {
            let command = trainaxonml(fastafile, label, epochs).unwrap();
            println!("The deep learning results have been written: {}", command);
        }
    }
}
