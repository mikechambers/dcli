# dcli Release Notes

## v0.98.0 March 3 2023

-   Added support for Lightfall (lightfall), and Season of Definance (season_of_defiance) moments.

## v0.97.0 January 20, 2023

-   Removed some workaround for API bugs for Competitive and IronBanner (API was fixed). IronBanner matches now appear.
-   Added support for RIFT_COMPETITIVE, SHOWDOWN_COMPETITIVE and SURVIVAL_COMPETITIVE modes. Before, you could only search for all competitive.
    -Recommended that you delete data store and resync from scratch.

## v0.96.0 December 25, 2022

-   Refactored database schema. This is major refactor that should make it easier to work with the data. It will require data to be re-synced.

## v0.95.1 December 18, 2022

-   Fix some issues, missing cases from v0.95.0 changes

## v0.95.0 December 17, 2022

-   Improved handling of missing character class data from API

## v0.94.0 December 13, 2022

-   Fix corrupted Rift matches (mode set to 0)

## v0.93.0 December 13, 2022

-   Added support for Iron Banner Zone Control (iron_banner_zone_control)
-   Added code that attempts to fix missing / incorrect mode data for some matches returned from API. This includes Season of the Seraph comp match data, as well as data for private matches. In order to get fixed data, you will need to delete your data store and resync all data.

## v0.91.0 December 6, 2022

-   Update required Rust compiler version to 1.65.0 and updated libraries to most recent versions
-   Changed SEASON_OF_THE_PLUNDER moment to SEASON_OF_PLUNDER (season_of_plunder)
-   Fixed --verbose flag not working in dclisync
-   Added support for Season of the Seraph

## v0.9.0 November 12, 2022

-   Added a daemon mode to dclisync which continuously checks for updated data with a pause between syncs
-   dclisync now listens for Ctrl-C / SIGINT / SIGTERM events, and will try to gracefully shutdown
-   Updated progress output / visuals (now show a progress bar and spinner)
-   Refactored all app console output. (please report any issues)

## v0.8.91 November 1, 2022

-   Workaround issue where system could get in state where it would not clear a previous activity from the queue, forcing a reload of all data.

## v0.8.9 November 1, 2022

-   dclisync when adding members via group-id / clan, skip members who do not have a valid Bungie name / code.
-   Lots of refactoring to clean up code and improve performance
-   dclistat should be significantly faster when querying over long time periods
-   dcliad, dcliah and dclistat --character_class_selection now defaults to all from last_active

## v0.8.8 August 26, 2022

-   Better recovery when api calls fail when retrieving list of activities when syncing

## v0.8.7 August 23, 2022

-   Added support for players playing through Epic games platform

## v0.8.6 August 23, 2022

-   Added dclistat app to query specific stats.
-   Added new Moment for Season of the Plunder ("season_of_plunder").
-   Replaced --no-sync option in dcliad and dcliah with --sync, and no longer sync data by default.

## v0.8.5 August 7, 2022

-   Improved how corrupted data from Bungie is handled. Now, if data is corrupted, we check for existing data before updating. If the DCLI_FIX_DATA environment variable is set to true, then if there is not already a local, non-corrupt version of the data, we will go back to Bungie to retrieve updated data. Note, this can significantly slow down initial syncs.

## v0.8.4 August 1, 2022

-   Fixed data parsing error that could occur due to wrong data type in code
-   Fixed crash that could occur in verbose mode with http responses containing unicode characters.

## v0.8.3 July 7, 2022

-   Fixed crashed when viewing an activity using Glaives. Thank you @BinarMorker!

## v0.8.2 May 31, 2022

-   Added support for Iron Banner Rift ("iron_banner_rift") and Rift modes.
-   Added support for Season of the Haunted ("season_of_the_haunted") moment.

## v0.8.01 February 18, 2022

-   Added season_of_the_risen, witch_queen moments.

## v0.8.0 January 15, 2022

-   Update database to store additional game data (emblem hash for players and fireteamId). This will require a database update and data to be resynced
-   Use new bungie api to search for player name, which fixes issue of some names not being able to be synced
-   Renamed dclias to dclisync
-   Major additions to dclisync, including ability to manage players who are being synced, and sync all players at once
-   Some signficant data handling refactoring. Please report any bugs / issues
-   Added flags to specify your own Destiny API key (required for dclisync)

## v0.7.2 December 3, 2021

-   Fixed bug where Elimination mode was not supported

## v0.7.1 November 2, 2021

-   Added support for Iron Banner gold medals

## v0.7.0 October 31, 2021

-   Deprecated and removed dclics, dclims, dclic and dclis
-   Added support for specifying player via Bungie names
-   Removed support for specifying player by member id and platform (use Bungie name instead)
-   Added support for displaying medal info in dcliah
-   Added table in datastore that tracks which players have been synced

## v0.6.3 September 5, 2021

-   Added support / moment for Season of the Lost (season_of_the_lost)
-   Update compiler version to 1.53.0
-   Updated all crates to latest version

## v0.6.2 June 21, 2021

-   Added support for tracking data for multiple players. This requires all data to be re-synced.
-   Fixed bug which could cause activity sync to get in infinite loop if error occured when saving activity data.
-   Added rust-toolchain.toml to force 1.50.0 version of rust when compiling. More recent versions of the rust compiler break some of the packages used and will not compile.
-   dcliah : Included additional information on character and class which stats are displayed for.
-   dcliah : Now display win percentage for weapons in games where there is a kill with the weapon.
-   dcliah : removed kills percent total of all games weapon data.
-   Fixed incorrect weekly, daily reset times (was an hour late).

## v0.5.63 June 11, 2021

-   Added Season of the Splicer moment.
-   Fixed typo for Scout Rifles. (Thanks alexcpsec for the patch).

## v0.5.62 February 19, 2021

-   Fixed bug that could cause some activities to never sync property (and could throw RowNotFound error.) Requires all data to be re-synced.
-   Fixed issues where errors would occur if new data is found in API, and manifest hasnt been updated yet.
-   Updated required compiler version (1.50.0), and a number of libraries (tokio, sqlx, reqwest).

## v0.5.61 February 7, 2021

-   Added moment for Season of the Chosen.

## v0.5.6 February 7, 2021

-   Display mercy data in dcliah.

## v0.5.5 January 24, 2021

-   Added player rating to dcliad (this is based on Destiny combat rating, similar to elo).
-   Fixed bug where wrong platform was being stored for players. Requires database update and data to be re-synced.
-   Added --end-moment argument to dcliah to allow specifying start / end moments to search.
-   Added moments for each season (see docs for [dcliah](https://github.com/mikechambers/dcli/tree/main/src/dcliah)).
-   Some performance optimizations for data store queries.

## v0.5.1 January 22, 2021

-   Re-releasing as Windows Defender falsly flagged dcliah as containing malware (known issue with Defender). Hoping it won't flag new build.

## v0.5.0 January 22, 2021

-   Released dcliad (per-game details).
-   Removed dclics (last included in v0.3.71).
-   removed stats_report example (required dclics).
-   Updated data store format. Will require all data to be re-downloaded.
-   Data for all activity players is now stored in data store (previously only stored data for specified player).
-   dcliah added some additional data to stat output, including --activity-index, which can be used to load game specific data in dcliad.

## v0.3.71 January 14, 2021

-   Fixed bug that led to wrong weapon stats when used in 1 game.
-   Fixed bug preventing import of data of multiple players when they had played in same activity.

## v0.3.7 : January 11, 2021

-   Added additional weapon metrics.

## v0.3.6 : January 6, 2021

-   Added support to sort weapon stat results in dcliah (--weapon-sort).

## v0.3.5 : January 6, 2021

-   Added support for private matches. See [issue 10](https://github.com/mikechambers/dcli/issues/10) for current limitations.
-   General optimizations and performance improvements.
-   This update will require that all activity data be redownloaded.

## v0.3.2 : January 1, 2021

-   Fixed wrong version numbers in some apps.
-   Updated Copyright year.

## v0.3.1 : December 30, 2020

-   Updated tests to run with latest release.

## v0.3.0 : December 30, 2020

-   Refactored dcliah to use local data.
-   Added dclisync.
-   Deprecated dclics.
-   Added default storage locations for data storage. No longer need to specify manifest-dir for apps.
-   Simplified and standardized argument names. Please review docs.

## v0.2.0 : December 14, 2020

-   Added dcliah.
-   Added dclitime.
-   Updated and standardized all tool command line arguments (see docs).
-   Added examples/session (bash and Windows Powershell).
-   Lots of fixes and optimizations.

## v0.1.1 : December 3, 2020

-   Fix [#1](https://github.com/mikechambers/dcli/issues/1).
-   Fix [#2](https://github.com/mikechambers/dcli/issues/2).
-   Fix [#3](https://github.com/mikechambers/dcli/issues/3).
-   Updated [status_notification_win.ps1](https://github.com/mikechambers/dcli/blob/main/examples/status_notification_win.ps1). Requires dclia v0.1.1.

## v0.1.0 : December 2, 2020

-   Initial release.
-   Includes:
    -   dclis
    -   dclim
    -   dclic
    -   dclims
    -   dclia
    -   dclics
