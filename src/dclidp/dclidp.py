#!/usr/bin/env python3

# Copyright (c) 2020 Mike Chambers
# Released under an MIT License
#
# https://github.com/mikechambers/dcli
# 
# dclidp.py : Command line program for syncing and managing
# the manifest database for the Destiny 2 API

import argparse
import urllib.request
import json
import ssl
import zipfile
import io
import os.path
import sys

VERSION = "0.85"

MANIFEST_URL = "https://www.bungie.net/Platform/Destiny2/Manifest/"
BASE_URL = "https://www.bungie.net"

INFO_NAME="manifest-info.json"
MANIFEST_NAME="manifest.sqlite"

#constants for command         #TODO: note its adding extra stringsline flags
LOCAL_VERSION="local.version"
LOCAL_URL="local.url"
REMOTE_VERSION="remote.version"
REMOTE_URL="remote.url"


def main():
    parser = argparse.ArgumentParser(description="Manage local and remote Destiny 2 API Manifest Database")

    parser.add_argument("--key", "-k", 
                                    required=True, help="Destiny 2 API Key")
    parser.add_argument("--manifest_dir", "-o",
                            required=True, help="Directory to store manifest")
    parser.add_argument('--version', 
                                action='version', version='%(prog)s ' + VERSION)

    parser.add_argument('--info', 
            help="Print info on local or remote manifest.", 
            choices=[LOCAL_VERSION, LOCAL_URL, REMOTE_VERSION, REMOTE_URL])
    
    parser.add_argument("--check", help="Check whether there is an updated remote manifest file.", action="store_true")

    parser.add_argument("--force", help="Force download and update of manifest file.", action="store_true")

    args = parser.parse_args()

    #Bungie / Destiny 2 API Key
    key = args.key

    #directory where we will store maniinfo_pathfest and manifest info files.
    manifest_dir = args.manifest_dir

    #leave it up to the user to pass in a dir that already exists
    if os.path.exists(manifest_dir) == False:
        print("Error : --manifest_dir does not exists")
        sys.exit(1)

    #path to the manifest db file
    manifest_path = os.path.join(manifest_dir, MANIFEST_NAME)
    manifest_exists = os.path.exists(manifest_path)

    #path to the manifest info json file
    manifest_info_path = os.path.join(manifest_dir, INFO_NAME)
    info_file_exists = os.path.exists(manifest_info_path)

    if info_file_exists:
        #read info file if it exists
        local_manifest_info = get_local_manifest_info(manifest_info_path)

    #retrieve remote manifest info from bungie API
    remote_manifest_info = retrieve_manifest_info(key)    
        
    #check if request is to just get manifest info
    #if local data is requested, and it doesnt exist, we just quietly
    #exit    
    if args.info != None:
        if args.info == LOCAL_VERSION:
            if info_file_exists == False or manifest_exists == False:
                sys.exit(0)
            print(local_manifest_info.version)
        elif args.info == LOCAL_URL:
            if info_file_exists == False or manifest_exists == False:
                sys.exit(0)
            print(local_manifest_info.url)
        elif args.info == REMOTE_URL:
            print(remote_manifest_info.url)
        elif args.info == REMOTE_VERSION:
            print(remote_manifest_info.version)
        
        sys.exit(0)

    updated_manifest_is_available = False
    if info_file_exists == False or manifest_exists == False:
        #if local manifest or info file are missing
        updated_manifest_is_available = True
    else:
        #compare manifest URLs to see if tare different (and thus 
        # remote has been updated)
        updated_manifest_is_available = (local_manifest_info.url !=
                                                remote_manifest_info.url)

    if args.check:
        if updated_manifest_is_available:
            print("Updated manifest found")
            print("version  : " + remote_manifest_info.version)
            print("url      : " + remote_manifest_info.url)
        else:
            print("No new manifest version found")

        sys.exit(0) 

    
    #if we havent found an updated manifest and are not forcing and update
    if updated_manifest_is_available == False and args.force == False:
        sys.exit(0)

    try:
        #download and unzip the remote manifest DB file
        f = download_manifest(remote_manifest_info, manifest_dir, key)
        os.rename(f, manifest_path)
    except Exception:
        print("Error downloading and saving manifest file : " + manifest_path)
        print("Using manifest URL : " + remote_manifest_info.url)
        print(sys.exc_info())
        sys.exit(1)

    try:
        #write out the manifest info file
        write_manifest_info(remote_manifest_info, manifest_info_path)
    except Exception:
        print("Error saving manifest info file : " + manifest_info_path)
        print(sys.exc_info())
        sys.exit(1)

#load local manifest info
def get_local_manifest_info(manifest_info_path):
    with open(manifest_info_path) as f:
        j_data = json.load(f)

    info = ManifestInfo(BASE_URL)
    info.fromJson(j_data)
    return info

#downloads and uncompresses the manifest sqlite file
#returns path to uncompressed manifest file
def download_manifest(manifest_info, manifest_dir, key):
    url = manifest_info.url
    bytes = retrieve_url(url, key)
    z = zipfile.ZipFile(io.BytesIO(bytes))

    names = z.namelist()

    #name of file in manifest
    manifest_name = names[0]

    #write out file to file system
    f = z.extract(manifest_name, manifest_dir)

    #path to uncompressed file
    return f

#write out manifest info to file system
def write_manifest_info(manifest_info, output_path):
    data = manifest_info.toJson()

    with open(output_path, 'w') as f:
        #dump writes to a file
        f.write(data)

#load manifest data from bungie api
def retrieve_manifest_info(key):
    body = retrieve_url_string(MANIFEST_URL, key)
    j = json.loads(body)

    info = ManifestInfo(BASE_URL)
    info.parse(j)
    return info

#retrieves raw bytes from a URL request
def retrieve_url(url, key=None):

    if(key != None):
        headers={"X-API-Key":key}
        request = urllib.request.Request(url, headers=headers)
    else:
        request = urllib.request.Request(url)

    response = urllib.request.urlopen(request)
    bytes = response.read()
    return bytes

#retrieves body of a url request as a string
def retrieve_url_string(url, key=None):
    bytes = retrieve_url(url, key)
    body = bytes.decode("utf8")

    return body

#class to work with manifest info data
class ManifestInfo:
        
    def __init__(self, base_url):
        self.version = None
        self.url = None
        self.base_url = base_url

    #parses remote manifest json
    def parse(self, jData):
        self.version = jData["Response"]["version"]
        self.url = self.base_url + jData["Response"]["mobileWorldContentPaths"]["en"]

    #parses local manifest json
    def fromJson(self, infoData):
        self.version = infoData["version"]
        self.version = infoData["url"]

    #creates local manifest json
    def toJson(self):
        out = {}    
        out["version"] = self.version
        out["url"] = self.url

        #dumpS writes to a string
        j = json.dumps(out)
        return j

main()