# dcli examples

This folder contains scripts and examples that demonstrate using the data from the dcli apps.

If you have examples you would like to share you can:
* Submit them via the [issues page](https://github.com/mikechambers/dcli/issues)
* Submit them via a pull request
* Share a link to a page or repository with the example

Please provide a summary including any requirements (such as OS or shell environment), and make sure to comment your script.

All examples in hosted in this project are released under an MIT license.

## Examples

### [stats_report](stats_report)

Bash script that reads crucible stat data from dclics, and outputs a summary (will speak it if running on OS X). Demonstrates how to parse TSV name / value data from dcli apps into bash variables.

Requires a modern / updated version of bash to run on OS X. More info [here](https://itnext.io/upgrading-bash-on-macos-7138bd1066ba).

### status_notification

* [status_notification_osx](status_notification_osx)
* [status_notification_win.ps1](status_notification_win.ps1)

Mac OS X Bash and Windows Powershell scripts script which monitors a player's Destiny 2 status, and sends a notification with info on the new status when it changes.

This is particularly useful on Windows when playing Crucible, as it will display a notification as you load into the map, telling you which map you are loading into.

### [mail_report](mail_report)

Bash script that uses [dclics](https://github.com/mikechambers/dcli/tree/main/src/dclics) to generate and send an email report of weekly Crucible stats. Can be scheduled as part of a crontab job to automate sending.

Requires that a sendmail client is [configured](https://blog.travismclarke.com/post/send-email-from-terminal/) on system (although that should be easy to change in the scripts). Requires a newer version of Bash, so you may need to upgrade if running on OS X. Read script for configuration info.

### Snippets

#### Grab and print a single value

Prints out K/D for all modes for the past month:
```
$ dclics --member-id 4611686018429783292 --platform xbox --mode all --character-id 2305843009264966985 --period month --output tsv | grep kills_deaths_ratio | awk '{print $2}'
1.5986928
```
