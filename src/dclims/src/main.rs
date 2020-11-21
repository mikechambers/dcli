use exitfailure::ExitFailure;
use std::path::PathBuf;
use structopt::StructOpt;

use dcli::error::Error;
use dcli::utils::{print_error, print_standard};
use dcli::manifestinterface::{ManifestInterface, FindResult};

#[derive(StructOpt)]
/// Command line tool for searching the Destiny 2 manifest by hash ids.
///
/// Command line tool for searching the Destiny 2 manifest by hash ids.
/// Takes a hash / id from the Destiny 2 API, and returns data from the
/// item from the manifest.
struct Opt {
    ///Local path the Destiny 2 manifest database file.
    #[structopt(short = "m", long = "manifest-path", parse(from_os_str))]
    manifest_path: PathBuf,

    ///terse output in the form of name:description . Errors are suppresed.
    #[structopt(short = "t", long = "terse", conflicts_with = "verbose")]
    terse: bool,

    ///Output raw json data from manifest.
    #[structopt(short = "j", long = "json", conflicts_with = "verbose", conflicts_with="terse")]
    json: bool,

    ///Print out additional information for the API call
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    ///The hash id from the Destiny 2 API for the item to be searched for. Example : 326060471
    #[structopt(long = "hash", required = true)]
    hash: u32,
}

async fn search_manifest_by_hash(hash: u32, manifest_path: PathBuf) -> Result<Vec<FindResult>, Error> {
    let mut manifest = ManifestInterface::new(manifest_path, false).await?;
    let out = manifest.find(hash).await?;

    Ok(out)
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let opt = Opt::from_args();

    let results:Vec<FindResult> = match search_manifest_by_hash(opt.hash, opt.manifest_path).await {
        Ok(e) => e,
        Err(e) => {
            print_error(&format!("Could not search manifest : {:#}", e), !opt.terse);
            std::process::exit(1);
        }
    };

    if results.is_empty() {
        print_standard("No items found.", !opt.terse);
        std::process::exit(0);
    }

    //default prints name
    //verbose prints name and additional information
    //--json outputs json
    //do we need --terse
    //name:descrpiption:hasicon:iconpath

    for r in results.iter() {
        print_standard(&format!("Name : {}",r.display_properties.name), !opt.terse && !opt.json);
        print_standard(&format!("Description : {}",r.display_properties.description), !opt.terse && !opt.json);
        print_standard(&format!("Has Icon : {}",r.display_properties.has_icon), opt.verbose && !opt.terse && !opt.json);
        print_standard(&format!("Icon Path : {}",r.display_properties.icon), opt.verbose && !opt.terse && !opt.json);

        print_standard(&r.raw_json, opt.json);

        print_standard(&format!("{}:{}", &r.display_properties.name, &r.display_properties.description), opt.terse);
    }

    Ok(())
}
