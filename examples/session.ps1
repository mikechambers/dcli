#
# Tracks and displays combined crucible stats per gameplay session.
#
# Works on: Windows with Powershell
# If you have permission issues running, see:
# https://stackoverflow.com/a/62403405/10232
#
# Created by Mike Chambers
# https://www.mikechambers.com
#
# Released under an MIT License
# More info at:
# https://github.com/mikechambers/dcli/
#
# Requires dclia v0.1.1


################ Script configuration #################


#run dclim --manifest-path /tmp/
#to sync manifest before running this script
$manifest_path="/tmp/manifest.sqlite3"

#pull setting from environment variables. you can also
#just enter them here

#you can get member_id and platform by running dclis
$member_id=$env:MEMBER_ID
$platform="$env:PLATFORM

#can get character id from dclic
$character_id=$env:CHARACTER_ID

$mode="all_pvp"
$moment="now"

$session_start = (dclitime.exe --moment $moment)

$check_interval_seconds=30

############# program #############

$last_call_was_error=false
while ($true) {

    # assumes dclia is in your path
    $activity = (dclia --manifest-path $manifest_path --member-id $member_id --platform $platform --character-id $character-id --mode $mode --moment custom --start-time $session_start) -join "`n"
   
    if(!$?) {
	if(!$last_call_was_error) {
	    Write-Ouput "\nError retrieving activities. Trying again in{0} seconds." -f $check_interval_seconds
	    $last_call_was_error=true
	}
    } else {
	$last_call_was_error=false
   	clear
	Write-Output $activity
    }
    Start-Sleep -Seconds $check_interval_seconds
}
