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
# Requires dcliah and dclitime v0.2.0


################ Script configuration #################


#run dclim --manifest-path /tmp/
#to sync manifest before running this script
$manifest_path="/tmp/manifest.sqlite3"

#pull setting from environment variables. you can also
#just enter them here

#you can get member_id and platform by running dclis
$member_id=$env:MEMBER_ID
$platform=$env:PLATFORM

#can get character id from dclic
$character_id=$env:CHARACTER_ID

#for tracking trials on the weekend mode=trials_of_osiris moment=weekend
$mode="all_pvp"
$moment="now"

$session_start = (dclitime.exe --moment $moment)

$check_interval_seconds=30

############# program #############
Clear-Host
Write-Output "Retrieving activity data..."
$last_call_was_error=$false
while ($true) {

    # assumes dcliah.exe is in your path
	$activity = (dcliah.exe --manifest-path $manifest_path `
		--member-id $member_id --platform $platform --character-id $character_id `
		--mode $mode --moment custom --custom-time $session_start 2>$null)  -join "`n"
	#note, to view any errors that might occur, remove 2>$null (this will print
	#extra output though, or change to 2>err.txt and it will write to a text file)
	
    if($LASTEXITCODE) {
		if(!$last_call_was_error) {
			Write-Host ("Error retrieving activities. Trying again in {0} seconds" -f $check_interval_seconds) -ForegroundColor White -BackgroundColor Red
			$last_call_was_error=$true
		}
    } else {
		$last_call_was_error=$false
   		Clear-Host
		Write-Output $activity
    }
    Start-Sleep -Seconds $check_interval_seconds
}
