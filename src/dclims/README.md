# dclims

Command line tool for searching the Destiny 2 manifest by hash ids (from API calls).


## USAGE
```
USAGE:
    dclims [FLAGS] --hash <hash> --manifest-path <manifest-path>

FLAGS:
    -h, --help       
            Prints help information

    -t, --terse      
            terse output in the form of class_name:character_id . Errors are suppresed

    -V, --version    
            Prints version information

    -v, --verbose    
            Print out additional information for the API call


OPTIONS:
        --hash <hash>                      
            The hash id from the Destiny 2 API for the item to be searched for. Example : 326060471

    -m, --manifest-path <manifest-path>    
            Local path the Destiny 2 manifest database file

```

### Example

Retrieve information for *Luna's Howl* by its API hash id.
```
dclims --manifest-path ~/tmp/manifest.sqlite3 --hash 3260604718
```

which returns:

```
Name : Luna's Howl
Description : "Guardians never die. But we don't forget those who do." —Lord Shaxx
```

You can get additional information by passing the `--verbose` flag:

```
dclims --manifest-path ~/tmp/manifest.sqlite3 --hash 3260604718 --verbose
```

which outputs:

```
Name : Luna's Howl
Description : "Guardians never die. But we don't forget those who do." —Lord Shaxx
Has Icon : true
Icon Path : https://www.bungie.net/common/destiny2_content/icons/ca86c130898a90ed19a0a317df8ab389.jpg
```

You can retrieve the complete json from the manifest by specifying the `--json` flag.

```
dclims --manifest-path ~/tmp/manifest.sqlite3 --hash 3260604718 --json
```

which outputs:

```
{"displayProperties":{"description":"\"Guardians never die. But we don't forget those who do.\" —Lord Shaxx","name":"Luna's Howl","icon":"/common/destiny2_content/icons/ca86c130898a90ed19a0a317df8ab389.jpg","hasIcon":true},"scope":0,"sourceString":"Source: Reach a Glory rank of \"Fabled\" in the Crucible.","sourceHash":2537301256,"itemHash":153979396,"acquisitionInfo":{"acquireMaterialRequirementHash":130662630,"runOnlyAcquisitionRewardSite":false},"stateInfo":{"requirements":{"entitlementUnavailableMessage":""}},"presentationNodeType":2,"traitIds":[],"traitHashes":[],"parentNodeHashes":[1956740204],"hash":3260604718,"index":4967,"redacted":false,"blacklisted":false}
```

JSON output and format will vary depending on the item / information retrieved. 

## Compiling

This utility is written and compiled in [Rust](https://www.rust-lang.org/).

When compiling you must have an environment variable named `DESTINY_API_KEY` which contains your [Bungie API key](https://www.bungie.net/en/Application).