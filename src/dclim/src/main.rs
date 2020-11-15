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

use exitfailure::ExitFailure;
use structopt::StructOpt;
use dcli::apiclient::{ApiClient, ApiCallError, ApiCallErrorType, DestinyResponseStatus};
use dcli::manifest::{Manifest, ManifestResponse};


use std::path::PathBuf;
use std::env::current_dir;

use serde_derive::{Deserialize, Serialize};
use serde::{Deserialize, Deserializer};

fn load_manifest_info(manifest_info_path:&PathBuf) -> Option<ManifestInfo> {
    let m = ManifestInfo {
        url:String::from(""),
        version:String::from(""),
    };

    Some(m)
}

#[derive(Serialize, Deserialize, Debug)]
struct ManifestInfo {
    version:String,

    url:String,
}

impl ManifestInfo {

    fn from_manifest(manifest:&Manifest) -> ManifestInfo {
        ManifestInfo {
            version:String::from(&manifest.version),
            url:String::from(&manifest.mobile_world_content_paths.en),
        }
    }

    fn from_json(json:&str) -> ManifestInfo {

        //TODO: handle error?
        let m:ManifestInfo = serde_json::from_str(json).unwrap();

        m
    }

    fn to_json(&self) -> String {

        //todo: do wes need to catch errors here? Would this ever fail?
        serde_json::to_string(self).unwrap()
    }
}


async fn retrieve_manifest_info(print_url:bool) -> Result<ManifestInfo, ApiCallError> {

    let client:ApiClient = ApiClient::new(print_url);
    let url = "https://www.bungie.net/Platform/Destiny2/Manifest/";

    //custom header
    //TODO: handle parsing errorcontent
    let resp = match client.call_api(url).await {
        Ok(e) => e,
        Err(e) => {
            return Err(ApiCallError {
                message: format!("{}", e),
                _error_type: ApiCallErrorType::Request,
            })
        }
    };

    let response = match resp.json::<ManifestResponse>().await {
        Ok(e) => e,
        Err(e) => {
            return Err(ApiCallError {
                message: format!("{}", e),
                _error_type: ApiCallErrorType::Parse,
            })
        }
    };

    let m_info:ManifestInfo = ManifestInfo::from_manifest(&response.manifest);

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

//TODO: we can create an ErrorType enum for our struct if we need more info
struct Error {
    message:String,
}

fn get_manifest_dir(dir:&PathBuf) -> Result<PathBuf, Error> {

    //TODO: clean up these unwraps
    let c_dir = current_dir().unwrap();
    let m_dir = c_dir.join(dir.as_path());

    if m_dir.is_file() {
        println!("Error creating directory. File exists.");
        std::process::exit(0);
    }

    if !m_dir.exists() {
        let _m = match std::fs::create_dir_all(&m_dir) {
            Ok(e) => e,
            Err(_e) => {
                return Err(
                    Error{ 
                        message: String::from("Error creating directory."),
                    }
                );
            },
        };
    }

    //TODO: do we need to do this step?
    let m_dir = match std::fs::canonicalize(&m_dir.as_path()) {
        Ok(e) => e,
        Err(e) => {
            return Err(
                Error{ 
                    message:String::from("Error creating canonicalized path."),
                }
            );
        },
    };

    Ok(m_dir)
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let opt = Opt::from_args();

    let m_dir = match get_manifest_dir(&opt.manifest_dir) {
        Ok(e) => e,
        Err(e) => {
            println!("{}", e.message);
            println!("{:?}", opt.manifest_dir);
            std::process::exit(0);
        },
    };

    let m_path = m_dir.join("manifest.sqlite3");
    let m_info_path = m_dir.join("manifest_info.json");

    println!("{:?}", m_path);
    println!("{:?}", m_info_path);

    let mut force = opt.force;
    if !force && (!m_path.exists() || !m_info_path.exists()) {
        force = true;
    }

    //TODO: unwrap this
    let remote_manifest_info = match retrieve_manifest_info(opt.url).await {
        Ok(e) => e,
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(0);
        },
    };

    let m = ManifestInfo::from_json(&remote_manifest_info.to_json());
    println!("{:?}", &m);

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
