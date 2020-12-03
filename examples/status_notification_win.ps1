
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


$check_interval_seconds=15

$member_id="00000000000000000000"
$platform="xbox"

#run dclim --manifest-path /tmp/
#to sync manifest before running this script
$manifest_path="/tmp/manifest.sqlite3"

$old_activity=""

while ($true) {


    $activity = (dclia --manifest-path $manifest_path --member-id $member_id --platform $platform) -join "`n"

    #note the inconsistent use of trailing period. Ill fix this in the app
    $skip_notification = (($activity -eq "Not currently in an activity.") -or ($activity -eq "Currently sitting in Orbit"))
    if ( ($old_activity -ne $activity) -and  !$skip_notification)
    { 
        [reflection.assembly]::loadwithpartialname("System.Windows.Forms")
        [reflection.assembly]::loadwithpartialname("System.Drawing")
        $notify = new-object system.windows.forms.notifyicon
        $notify.icon = [System.Drawing.SystemIcons]::Information
        $notify.visible = $true
        $notify.showballoontip(10,"Destiny 2 Activity Changed",$activity,[system.windows.forms.tooltipicon]::None)

        $old_activity = $activity

        #TODO: when we find a new activity, lets delay the next check for a bit, in case the server api returns old data
        # for a short period
    }

    Start-Sleep -Seconds $check_interval_seconds
}