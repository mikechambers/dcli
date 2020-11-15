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

use dcli::apiclient::ApiClient;
use dcli::error::Error;
use dcli::manifest::{Manifest, ManifestResponse};
use exitfailure::ExitFailure;
use structopt::StructOpt;

use std::fs;

use std::env::current_dir;
use std::path::PathBuf;

use serde_derive::{Deserialize, Serialize};

fn _load_manifest_info(_manifest_info_path: &PathBuf) -> Option<ManifestInfo> {
    let m = ManifestInfo {
        url: String::from(""),
        version: String::from(""),
    };

    Some(m)
}

//TODO: move to its own file
#[derive(Serialize, Deserialize, Debug)]
struct ManifestInfo {
    version: String,

    url: String,
}

impl ManifestInfo {
    fn from_manifest(manifest: &Manifest) -> ManifestInfo {
        ManifestInfo {
            version: String::from(&manifest.version),
            url: String::from(&manifest.mobile_world_content_paths.en),
        }
    }

    fn from_json(json: &str) -> Result<ManifestInfo, Error> {
        let m: ManifestInfo = serde_json::from_str(json)?;

        Ok(m)
    }

    fn to_json(&self) -> Result<String, Error> {
        //todo: do wes need to catch errors here? Would this ever fail?
        let out = serde_json::to_string(self)?;

        Ok(out)
    }
}

async fn retrieve_manifest_info(print_url: bool) -> Result<ManifestInfo, Error> {
    let client: ApiClient = ApiClient::new(print_url);
    let url = "https://www.bungie.net/Platform/Destiny2/Manifest/";

    let response = client.call_and_parse::<ManifestResponse>(url).await?;

    let m_info: ManifestInfo = ManifestInfo::from_manifest(&response.manifest);

    Ok(m_info)
}

#[derive(StructOpt)]
/// Command line tool for retrieving and managing the Destiny 2 manifest database.
///
///
struct Opt {
    #[structopt(short = "d", long = "dir", required = true, parse(from_os_str))]
    manifest_dir: PathBuf,

    ///Print out the url used for the API calls
    #[structopt(short = "u", long = "url")]
    url: bool,

    ///Force a download of manifest regardless of whether it has been updated
    #[structopt(short = "f", long = "force")]
    force: bool,
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

    println!("{:?}", m_path);
    println!("{:?}", m_info_path);

    let mut force = opt.force;
    if !force && (!m_path.exists() || !m_info_path.exists()) {
        force = true;
    }

    //TODO:TEMP UNTIL MANIFEST SAVING IS IN
    force = false;

    println!("Force : {}", force);

    //TODO: unwrap this
    let remote_manifest_info = match retrieve_manifest_info(opt.url).await {
        Ok(e) => e,
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(0);
        }
    };

    if !force {
        //maybe make this an Option
        if let Ok(e) = load_manifest_info(&m_info_path) {
            let local_manifest_info: ManifestInfo = e;
            println!("{:?}", local_manifest_info);
        } else {
            force = true;
        };
    }

    //let m = ManifestInfo::from_json(&remote_manifest_info.to_json());
    //println!("{:?}", &m);
    /*
        match save_manifest_info(&m_info, &m_info_path) {
            Ok(e) => e,
            Err(e) => println!("Could not save manifest info"),
        };
    */

    /*
    if !force {
        //maybe make this an Option
        let local_manifest_info:ManifestInfo = load_manifest_info(&m_info_path).unwrap();
    }

    if(force || local_manifest_info.url != remote_manifest_info.url) {
        //download_manifest(m_path);
        //write_manifest_info(m_info_path, remote_manifest_info);
    }
    */

    println!("{:?}", m_dir.exists());

    Ok(())
}
