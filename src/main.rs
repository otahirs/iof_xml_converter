use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;
use clap::Parser;
use crate::models::iofxml::result_list::{PersonResult, ResultList};
use tracing::warn;

mod models;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// input file path
    #[arg(short, long)]
    input: PathBuf,

    /// output file path
    #[arg(short, long, default_value = "output.xml")]
    output: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let file = File::open(args.input)?;
    let reader = BufReader::new(file);
    let mut xml: ResultList = quick_xml::de::from_reader(reader)?;

    for mut class in &mut xml.class_result {
        let mut class_person_results = vec![];
        for team in &class.team_result {
            let team_member = match team.team_member_result.iter().next().cloned() {
                Some(t) => t,
                None => {
                    warn!("Team without team member result, bib_num: {:?}", team.bib_number);
                    continue;
                },
            };
            let person = match team_member.person {
                Some(p) => p,
                None => {
                    warn!("Team without person, bib_num: {:?}", team.bib_number);
                    continue;
                },
            };

            class_person_results.push( PersonResult {
                entry_id: team.entry_id.clone(),
                person,
                organisation: team.organisation.clone().and_then(|o| o.into_iter().next()),
                result: team_member.result
            })
        }
        class.person_result = class_person_results;
        class.team_result = vec![];
    }

    let res = quick_xml::se::to_string(&xml)?;
    File::create(args.output)?.write_all(res.as_bytes())?;

    Ok(())
}
