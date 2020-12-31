
:: Simple script that just runs all of the dcli apps on Windows
::
:: Created by Mike Chambers
:: https://www.mikechambers.com
::
:: Released under an MIT License
::
:: More info at:
:: https://github.com/mikechambers/dcli/

@echo off

echo ------------- RUNNING dclis.exe--------------

call dclis.exe --name mesh --platform xbox

echo ------------- RUNNING dclim.exe--------------

call dclim.exe

echo ------------- RUNNING dclic.exe--------------

call dclic.exe --member-id 4611686018429783292 --platform xbox

echo ------------- RUNNING dclims.exe--------------

call dclims.exe --hash 3260604718

echo ------------- RUNNING dclia.exe--------------

call dclia.exe --member-id 4611686018429783292 --platform xbox

echo ------------- RUNNING dclics.exe--------------

call dclics.exe --member-id 4611686018429783292 --platform xbox --character-id 2305843009264966985

echo ------------- RUNNING dcliah.exe --------------

dcliah.exe --member-id 4611686018429783292 --platform xbox

echo ------------- RUNNING dclitime.exe ---------------

call dclitime.exe


