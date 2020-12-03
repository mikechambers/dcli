
# Monitors changes in activity status and send notification when it changes
#
# Works on: Windows with Powershell
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

$old_activity

$condition = $true
while ($true) {


    $activity = (dclia --manifest-path ~/tmp/manifest.sqlite3 --member-id $member_id --platform $platform) -join "`n"
    if ( $old_activity -ne $activity )
    { 
        [reflection.assembly]::loadwithpartialname("System.Windows.Forms")
        [reflection.assembly]::loadwithpartialname("System.Drawing")
        $notify = new-object system.windows.forms.notifyicon
        $notify.icon = [System.Drawing.SystemIcons]::Information
        $notify.visible = $true
        $notify.showballoontip(10,"Destiny 2 Activity Changed",$activity,[system.windows.forms.tooltipicon]::None)

        $old_activity = $check_interval_seconds
    }

    Start-Sleep -Seconds 5
}