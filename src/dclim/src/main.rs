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

mod manifest_info;

use std::fs;
use std::path::PathBuf;

use dcli::apiclient::ApiClient;
use dcli::error::Error;
use dcli::manifestinterface::MANIFEST_FILE_NAME;
use dcli::output::Output;
use dcli::response::manifest::ManifestResponse;
use dcli::utils::EXIT_FAILURE;
use dcli::utils::{build_tsv, determine_data_dir, print_error, print_verbose};
use manifest_info::ManifestInfo;
use structopt::StructOpt;
use tokio::prelude::*;

pub const MANIFEST_INFO_FILE_NAME: &str = "manifest_info.json";

async fn retrieve_manifest_info(
    print_url: bool,
) -> Result<ManifestInfo, Error> {
    let client: ApiClient = ApiClient::new(print_url)?;
    let url = "https://www.bungie.net/Platform/Destiny2/Manifest/";

    let response = client.call_and_parse::<ManifestResponse>(url).await?;

    let manifest = match &response.response {
        Some(e) => e,
        None => return Err(Error::ApiResponseMissing), //we should never get here as this will be caught earlier
    };

    let m_info: ManifestInfo = ManifestInfo::from_manifest(&manifest);

    Ok(m_info)
}

fn save_manifest_info(
    manifest_info: &ManifestInfo,
    path: &PathBuf,
) -> Result<(), Error> {
    let json = manifest_info.to_json()?;

    //opens a file for writing. creates if it doesn't exist, otherwise
    //overwrites it
    fs::write(path, &json)?;

    Ok(())
}

fn load_manifest_info(path: &PathBuf) -> Result<ManifestInfo, Error> {
    let json = fs::read_to_string(path)?;
    let m = ManifestInfo::from_json(&json)?;

    Ok(m)
}

//should this move to ApiClient?
async fn download_manifest(
    url: &str,
    path: &PathBuf,
    print_url: bool,
) -> Result<(), Error> {
    let client: ApiClient = ApiClient::new(print_url)?;

    //Download the manifest
    let mut response = client.call(url).await?;

    //create a Vector to store the bytes for the download
    let mut out: Vec<u8> = Vec::new();

    //read of the bytes into the vector
    while let Some(chunk) = response.chunk().await? {
        out.write_all(&chunk).await?;
    }

    //create a cursor so we can move through the bytes
    let cursor = std::io::Cursor::new(out);

    //create a new zip archive from the bytes (since the download is compressed)
    let mut zip = zip::ZipArchive::new(cursor)?;

    //get a reference to the first file in the zip (there should only be one)
    let mut manifest = zip.by_index(0)?;

    //reference to file we are going to write the ucompressed manifest to
    let mut outfile = fs::File::create(&path)?;

    //save the uncompressed / unzipped manifest to the file system
    std::io::copy(&mut manifest, &mut outfile)?;

    Ok(())
}

#[derive(StructOpt, Debug)]
#[structopt(verbatim_doc_comment)]
/// Command line tool for retrieving and managing the Destiny 2 manifest database.
///
/// Manifest will be stored in the specified local directory with the file name:
/// manifest.sqlite3, along with meta-data with information about the downloaded
/// version. This is used to to determine whether the remote version has been updated.
///
/// Created by Mike Chambers.
/// https://www.mikechambers.com
///
/// Get support, request features or just chat on the dcli Discord server:
/// https://discord.gg/2Y8bV2Mq3p
///
/// Get the latest version, download the source and log issues at:
/// https://github.com/mikechambers/dcli
///
/// Released under an MIT License.
struct Opt {
    /// Directory where manifest will be stored. (optional)
    ///
    /// By default data will be loaded from and stored in the appropriate system
    /// local storage directory. Manifest will be stored in a sqlite3 database file
    /// named manifest.sqlite3
    #[structopt(short = "D", long = "data-dir", parse(from_os_str))]
    data_dir: Option<PathBuf>,

    ///Print out additional information
    ///
    ///Output is printed to stderr.
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    ///Force a download of manifest regardless of whether it has been updated.
    #[structopt(short = "F", long = "force", conflicts_with = "check")]
    force: bool,

    ///Check whether a new manifest version is available, but do not download.
    #[structopt(short = "C", long = "check")]
    check: bool,

    /// Format for command output
    ///
    /// Valid values are default (Default) and tsv.
    ///
    /// tsv outputs in a tab (\t) seperated format of name / value pairs with lines
    /// ending in a new line character (\n).
    #[structopt(
        short = "O",
        long = "output-format",
        default_value = "default"
    )]
    output: Output,
}
#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    print_verbose(&format!("{:#?}", opt), opt.verbose);

    let data_dir = match determine_data_dir(opt.data_dir) {
        Ok(e) => e,
        Err(e) => {
            print_error("Error initializing manifest directory.", e);
            std::process::exit(EXIT_FAILURE);
        }
    };

    let m_path = data_dir.join(MANIFEST_FILE_NAME);
    let m_info_path = data_dir.join(MANIFEST_INFO_FILE_NAME);

    let remote_manifest_info = match retrieve_manifest_info(opt.verbose).await {
        Ok(e) => e,
        Err(e) => {
            print_error("Could not retrieve manifest info from Bungie.", e);
            std::process::exit(EXIT_FAILURE);
        }
    };

    let col_w = 30;
    if opt.output == Output::Default {
        println!(
            "{:<0col_w$}{}",
            "Remote Manifest version",
            remote_manifest_info.version,
            col_w = col_w
        );
        println!(
            "{:<0col_w$}{}",
            "Remote Manifest url",
            remote_manifest_info.url,
            col_w = col_w
        );
    }

    let mut manifest_needs_updating = !m_path.exists() || !m_info_path.exists();

    if !manifest_needs_updating {
        if let Ok(e) = load_manifest_info(&m_info_path) {
            let local_manifest_info: ManifestInfo = e;

            if opt.output == Output::Default {
                println!(
                    "{:<0col_w$}{}",
                    "Local Manifest version",
                    local_manifest_info.version,
                    col_w = col_w
                );
                println!(
                    "{:<0col_w$}{}",
                    "Local Manifest url",
                    local_manifest_info.url,
                    col_w = col_w
                );
            }

            manifest_needs_updating =
                local_manifest_info.url != remote_manifest_info.url;
        } else {
            //couldnt load local manifest, so we will try and update
            manifest_needs_updating = true;

            print_verbose(
                "Could not load local manifest info. Forcing download.",
                opt.verbose,
            );
        }
    }

    if manifest_needs_updating && opt.output == Output::Default {
        println!(
            "{:<0col_w$}{}",
            "Updated manifest available",
            &remote_manifest_info.version,
            col_w = col_w
        );
    }

    if opt.check {
        match opt.output {
            Output::Default => {
                if !manifest_needs_updating {
                    println!("No new manifest avaliable.");
                }
            }
            Output::Tsv => {
                let mut name_values: Vec<(&str, String)> = Vec::new();
                name_values.push((
                    "update_avaliable",
                    format!("{}", manifest_needs_updating),
                ));
                name_values.push(("updated", format!("{}", false)));
                name_values.push(("version", remote_manifest_info.version));
                name_values.push(("url", remote_manifest_info.url));

                print!("{}", build_tsv(name_values));
            }
        }
        return;
    }

    if opt.force || manifest_needs_updating {
        //print to stderr so user can redirect other output (such as tsv) to stdout
        eprintln!("Downloading manifest. This may take a bit of time.");
        match download_manifest(&remote_manifest_info.url, &m_path, opt.verbose)
            .await
        {
            Ok(e) => e,
            Err(e) => {
                print_error("Could not download and save manifest", e);
                std::process::exit(EXIT_FAILURE);
            }
        };

        print_verbose("Download and save complete.", opt.verbose);
        print_verbose("Saving manifest info.", opt.verbose);

        match save_manifest_info(&remote_manifest_info, &m_info_path) {
            Ok(e) => e,
            Err(e) => {
                print_error("Could not save manifest.", e);
                std::process::exit(EXIT_FAILURE);
            }
        }

        if opt.output == Output::Default {
            println!("Manifest info saved.");
        }
    } else if opt.output == Output::Default {
        println!("No new manifest available");
    }

    match opt.output {
        Output::Default => {
            println!("{}", m_path.display());
        }
        Output::Tsv => {
            let mut name_values: Vec<(&str, String)> = Vec::new();
            name_values.push(("local_path", format!("{}", m_path.display())));
            name_values
                .push(("updated", format!("{}", manifest_needs_updating)));
            name_values.push(("version", remote_manifest_info.version));
            name_values.push(("url", remote_manifest_info.url));

            print!("{}", build_tsv(name_values));
        }
    }
}
