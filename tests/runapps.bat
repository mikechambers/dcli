
:: Simple script that just runs all of the dcli apps on Windows
::
:: Created by Mike Chambers
:: https://www.mikechambers.com
::
:: Released under an MIT License
::
:: More info at:
:: https://github.com/mikechambers/dcli/

:: make sure you place the app in your PATH

@echo off

echo ------------- RUNNING dclis.exe--------------

call dclis.exe --id mesh --platform xbox

echo ------------- RUNNING dclim.exe--------------

call dclim.exe --manifest-dir \tmp\

echo ------------- RUNNING dclic.exe--------------

call dclic.exe --member-id 4611686018429783292 --platform xbox

echo ------------- RUNNING dclims.exe--------------

call dclims.exe --manifest-path \tmp\manifest.sqlite3 --hash 3260604718

echo ------------- RUNNING dclia.exe--------------

call dclia.exe --manifest-path \tmp\manifest.sqlite3 --member-id 4611686018429783292 --platform xbox

echo ------------- RUNNING dclics.exe--------------

call dclics.exe --member-id 4611686018429783292 --platform xbox --mode all --character-id 2305843009264966985 --period week