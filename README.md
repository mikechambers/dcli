# dcli

dcli is a collection of utilities that provide a command line interface (CLI) for working with the [Destiny 2 API](https://github.com/Bungie-net/api). 

The apps require an API key from Bungie for working with the API. More info at: [https://www.bungie.net/en/Application](https://www.bungie.net/en/Application).

## Utilities

### dclid

Command line utilitie for managing and syncing the remote Destiny 2 API manifest database. The utility can be used to:

* Download latest version of the manifest sqlite database file
* Query whether the manifest has been updated since it was last downloaded
* Query the latest URL and version for the remote manifest

USAGE:
```
usage: dclid [-h] --key KEY --manifest_dir MANIFEST_DIR [--version]
             [--info {local.version,local.url,remote.version,remote.url}]
             [--check] [--force]
```


