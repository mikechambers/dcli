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

use manifest_info::ManifestInfo;

use exitfailure::ExitFailure;
use structopt::StructOpt;
use dcli::apiclient::{ApiClient, ApiCallError};

use std::path::PathBuf;
use std::env::current_dir;

fn load_manifest_info(manifest_info_path:&PathBuf) -> Option<ManifestInfo> {
    let m = ManifestInfo {
        url:String::from(""),
        version:String::from(""),
    };

    Some(m)
}

fn retrieve_manifest_info() -> Result<ManifestInfo, ApiCallError> {
    let m = ManifestInfo {
        url:String::from(""),
        version:String::from(""),
    };

    Ok(m)
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
    _url: bool,

    ///Force a download of manifest regardless of whether it has been updated
    #[structopt(short = "f", long = "force")]
    force: bool,
}

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

    //TODO: handle error
    let local_manifest_info:ManifestInfo = load_manifest_info(&m_info_path).unwrap();
    let remote_manifest_info = retrieve_manifest_info();

    /*
    if(force || local_manifest_info.url != remote_manifest_info.url) {
        //download_manifest(m_path);
        //write_manifest_info(m_info_path, remote_manifest_info);
    }
    */

    println!("{:?}", m_dir.exists());

    Ok(())
}
