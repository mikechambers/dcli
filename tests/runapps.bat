
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

call dclic.exe --name mesh#3230

echo ------------- RUNNING dclims.exe--------------

call dclims.exe --hash 3260604718

echo ------------- RUNNING dclia.exe--------------

call dclia.exe --name mesh#3230

echo ------------- RUNNING dcliah.exe --------------

dcliah.exe --name mesh#3230

echo ------------- RUNNING dcliad.exe --------------

dcliad.exe --name mesh#3230

echo ------------- RUNNING dclitime.exe ---------------

call dclitime.exe


