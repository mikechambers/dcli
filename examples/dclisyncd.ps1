
$check_interval_seconds = 30


############# program #############

while ($true) {

    $output = (dclisync --sync)

    Write-Output $output

    Start-Sleep -Seconds $check_interval_seconds
}