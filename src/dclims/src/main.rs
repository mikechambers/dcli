/*
* Copyright 2020 Mike Chambers
* https://github.com/mikechambers/dcli
*
* Permission is hereby granted, free of charge, to any person obtaining a copy of
* this software and associated documentation files (the "Software"), to deal in
* the Software without restriction, including without limitation the rights to
* use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
* of the Software, and to permit persons to whom the Software is furnished to do
* so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
* FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
* COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
* IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
* CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use std::path::PathBuf;
use structopt::StructOpt;

use dcli::error::Error;
use dcli::utils::{print_error, print_standard, EXIT_FAILURE};
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
async fn main() {
    let opt = Opt::from_args();

    let results:Vec<FindResult> = match search_manifest_by_hash(opt.hash, opt.manifest_path).await {
        Ok(e) => e,
        Err(e) => {
            print_error(&format!("Could not search manifest : {:#}", e), !opt.terse);
            std::process::exit(EXIT_FAILURE);
        }
    };

    if results.is_empty() {
        print_standard("No items found.", !opt.terse);
        return;
    }

    for r in results.iter() {

        let default : String = "".to_string();
        let description = r.display_properties.description.as_ref().unwrap_or(&default);

        print_standard(&format!("Name : {}",r.display_properties.name), !opt.terse && !opt.json);
        print_standard(&format!("Description : {}", description), !opt.terse && !opt.json);
        print_standard(&format!("Has Icon : {}",r.display_properties.has_icon), opt.verbose && !opt.terse && !opt.json);
        print_standard(&format!("Icon Path : {}",r.display_properties.icon_path.as_ref().unwrap_or(&default)), opt.verbose && !opt.terse && !opt.json);

        print_standard(&r.raw_json, opt.json);

        print_standard(&format!("{}:{}", &r.display_properties.name, description), opt.terse);
    }
}
