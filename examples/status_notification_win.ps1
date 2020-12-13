
# Monitors changes in activity status and send notification when it changes
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
$default_check_interval_seconds=15

#only check if Destiny process is running. Set to $false
#if you want it to check regardless of whether Destiny is running
#on the same machine
$only_check_if_destiny_running = $true

#whether it should print out status and other output to console
$quiet=$false

#you can get member_id and platform by running dclis
$member_id="00000000000000000000"
$platform="xbox"

#run dclim --manifest-path /tmp/
#to sync manifest before running this script
$manifest_path="/tmp/manifest.sqlite3"

############# program #############

$old_activity=""

while ($true) {

    $check_interval_seconds = $default_check_interval_seconds
    #check if Destiny is running and whether we should skip check if its not running
    $should_check = (Get-Process destiny2 -ErrorAction SilentlyContinue) -or !$only_check_if_destiny_running


    if ($should_check) {
        if (!$quiet) {
            Write-Output "Checking Status..."
        }

        # assumes dclia is in your path
        $activity = (dclia --manifest-path $manifest_path --member-id $member_id --platform $platform) -join "`n"
        $skip_notification = (($activity -eq "Not currently in an activity") -or ($activity -eq "Currently sitting in Orbit"))
    
        #dont send notification the first time we run
        if($old_activity -eq "") {

            if (!$quiet) {
                Write-Output $activity
                Write-Output "Initial status check. Skipping notification."
            }

            $skip_notification = $true
            $old_activity = $activity
        }

        if ( ($old_activity -ne $activity) -and  !$skip_notification)
        { 
            [void] [reflection.assembly]::loadwithpartialname("System.Windows.Forms")
            [void] [reflection.assembly]::loadwithpartialname("System.Drawing")
            $notify = new-object system.windows.forms.notifyicon
            $notify.icon = [System.Drawing.SystemIcons]::Information
            $notify.visible = $true
            $notify.showballoontip(10,"Destiny 2 Activity Changed",$activity,[system.windows.forms.tooltipicon]::None)
    
            if (!$quiet) {
                Write-Output $activity
            }

            $old_activity = $activity
            $check_interval_seconds = 60
        }
    } else {
        if (!$quiet) {
            Write-Output "Destiny 2 is not running. Skipping status check."
        }
    }

    Start-Sleep -Seconds $check_interval_seconds
}