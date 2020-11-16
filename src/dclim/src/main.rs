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

use dcli::apiclient::ApiClient;
use dcli::error::Error;
use dcli::manifest::ManifestResponse;
use exitfailure::ExitFailure;
use manifest_info::ManifestInfo;
use structopt::StructOpt;

use std::fs;

use std::env::current_dir;
use std::path::PathBuf;

async fn retrieve_manifest_info(print_url: bool) -> Result<ManifestInfo, Error> {
    let client: ApiClient = ApiClient::new(print_url);
    let url = "https://www.bungie.net/Platform/Destiny2/Manifest/";

    let response = client.call_and_parse::<ManifestResponse>(url).await?;

    let m_info: ManifestInfo = ManifestInfo::from_manifest(&response.manifest);

    Ok(m_info)
}

fn get_manifest_dir(dir: &PathBuf) -> Result<PathBuf, Error> {
    //TODO: clean up these unwraps
    let c_dir = current_dir().unwrap();
    let m_dir = c_dir.join(dir.as_path());

    if m_dir.is_file() {
        println!("Error creating directory. File exists.");
        std::process::exit(0);
    }

    if !m_dir.exists() {
        let _m = std::fs::create_dir_all(&m_dir)?;
    }

    //TODO: do we need to do this step?
    let m_dir = std::fs::canonicalize(&m_dir.as_path())?;

    Ok(m_dir)
}

fn save_manifest_info(manifest_info: &ManifestInfo, path: &PathBuf) -> Result<(), Error> {
    let json = manifest_info.to_json()?;

    //opens a file for writing. creates if it doesn't exist, otherwise
    //overwrites it
    fs::write(path, &json)?;

    Ok(())
}

fn load_manifest_info(path: &PathBuf) -> Result<ManifestInfo, Error> {
    //TODO: check if file exists? We shouldnt need to

    //TODO: make sure this error type is handled by error
    let json = fs::read_to_string(path)?;
    let m = ManifestInfo::from_json(&json)?;

    Ok(m)
}

//should this move to ApiClient?
async fn download_manifest(url: &str, path: &PathBuf, print_url:bool) -> Result<(), Error> {
    let client: ApiClient = ApiClient::new(print_url);

    let mut response = client.call(url).await?;

    use tokio::prelude::*;
    let mut out: Vec<u8> = Vec::new();

    while let Some(chunk) = response.chunk().await? {
        out.write_all(&chunk).await?;
    }

    let reader = std::io::Cursor::new(out);
    let mut zip = zip::ZipArchive::new(reader)?;

    let mut manifest = zip.by_index(0)?;

    let mut outfile = fs::File::create(&path)?;
    std::io::copy(&mut manifest, &mut outfile)?;

    Ok(())
}

#[derive(StructOpt)]
/// Command line tool for retrieving and managing the Destiny 2 manifest database.
///
///
struct Opt {
    #[structopt(short = "d", long = "dir", required = true, parse(from_os_str))]
    manifest_dir: PathBuf,

    ///Output additional information
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    ///Force a download of manifest regardless of whether it has been updated. Overrides --check.
    #[structopt(short = "f", long = "force")]
    force: bool,

    ///Check whether a new manifest version is available, but do not download. If overridden by --force.
    #[structopt(short = "c", long = "check")]
    check: bool,
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let opt = Opt::from_args();

    let m_dir = match get_manifest_dir(&opt.manifest_dir) {
        Ok(e) => e,
        Err(e) => {
            println!("{}", e);
            println!("{:?}", opt.manifest_dir);
            std::process::exit(0);
        }
    };

    let m_path = m_dir.join("manifest.sqlite3");
    let m_info_path = m_dir.join("manifest_info.json");

    let remote_manifest_info = match retrieve_manifest_info(opt.verbose).await {
        Ok(e) => e,
        Err(e) => {
            println!("Could not retrieve manifest info from Bungie : {}", e);
            std::process::exit(0);
        }
    };

    if opt.verbose {
        println!("Remote Manifest version : {}", remote_manifest_info.version);
        println!("Remote Manifest url     : {}", remote_manifest_info.url);
    }

    let mut manifest_needs_updating = !m_path.exists() || !m_info_path.exists();

    if !manifest_needs_updating {
        if let Ok(e) = load_manifest_info(&m_info_path) {
            let local_manifest_info: ManifestInfo = e;

            if opt.verbose {
                println!("Local Manifest version  : {}", local_manifest_info.version);
                println!("Local Manifest url      : {}", local_manifest_info.url);
            }

            manifest_needs_updating = local_manifest_info.url != remote_manifest_info.url;
        } else {
            //couldnt load local manifest, so we will try and update
            manifest_needs_updating = true;

            if opt.verbose {
                println!("Could not load local manifest info. Forcing download.");
            }
        }
    }

    if manifest_needs_updating {
        println!("Updated manifest available : {}", &remote_manifest_info.version);
    }

    if opt.check {
        if !manifest_needs_updating {
            println!("No new manifest avaliable.");
        }

        if !opt.force {
            std::process::exit(0);
        }
    }

    if opt.force || manifest_needs_updating {
        println!("Downloading manifest. This may take a bit of time.");
        match download_manifest(&remote_manifest_info.url, &m_path, opt.verbose).await {
            Ok(e) => e,
            Err(e) => {
                println!("Could not download and save manifest : {}", e);
                std::process::exit(0);
            }
        };

        if opt.verbose {
            println!("Download and save complete.");
            println!("Saving manifest info.");
        }

        match save_manifest_info(&remote_manifest_info, &m_info_path) {
            Ok(e) => e,
            Err(e) => {
                println!("Could not write manifest info : {}", e);
                std::process::exit(0);
            }
        }
        if opt.verbose {
            println!("Manifest info saved.");
        }

        println!("{}", m_path.display());
    } else {
        println!("No new manifest available");
    }

    Ok(())
}
