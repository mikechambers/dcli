# dclidp

This utility has been superseded by [dclim](https://github.com/mikechambers/dcli/tree/main/src/dclim).

Python 3 based Command line utility for managing and syncing the remote Destiny 2 API manifest database. The utility can be used to:

* Download latest version of the manifest sqlite database file
* Query whether the manifest has been updated since it was last downloaded
* Query the latest URL and version for the remote manifest

## USAGE
```
usage: dclidp.py [-h] --key KEY --manifest_dir MANIFEST_DIR [--version]
             [--info {local.version,local.url,remote.version,remote.url}]
             [--check] [--force]
```

The `--manifest_dir` argument is required, and should point a directory where the manifest and manifest info file can be saved anand maintained.

The `--key` argument is required and specifies the API key from Bungie.mesh
