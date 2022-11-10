## Service

This folder contains files to run *dclisync* as a system service using [systemctl](https://www.digitalocean.com/community/tutorials/how-to-use-systemctl-to-manage-systemd-services-and-units) on Linux based systems.

It consists of two files:


| FILE | DESCRIPTION |
| ---- | -----------  |
| dcli_sync_manager.py | A Python 3 based script that manages calls to dclisync
|
| dclisync.service | A systemctl service for dclisync |


### Installation

First, make sure that you have added the dcli apps to your system path, and have set up
the required environement variables).

Next, confirm that python 3 is available on your system by running:

```
$which python3
```
This should print out the path to the python3 executable (which we will need
below).

If it is not installed, install it.

Once python3 is installed, confirm the *dcli_sync_manager.py* script file can
run:

```
$python3 dcli_sync_manager.py
```

Once you configm that it is running you can stop the process / script.

Before installing the service, you must first edit the *dclisync.service* file.

```
[Unit]
Description=dclisync service
After=multi-user.target

[Service]
Type=simple
Restart=always
User=mesh
Group=mesh
ExecStart=/usr/bin/bash -lc "/usr/bin/python3 /home/mesh/bin/dcli_sync_manager.py"

[Install]
WantedBy=multi-user.target
```

Replacing the *User* and *Group* entries with the user and group you want the
service to run under (It does not require admin). It should be the same user
from which you call *dclim*. Then replace the paths to
*python3* and the *dcli_sync_manager.py* files to the correct paths.

At this point, your service should be ready to install and run. You can find
info [here](https://www.shubhamdipt.com/blog/how-to-create-a-systemd-service-in-linux/) on where to copy the file, and how to start / stop and monitor it.

Note, when adding new users, you should first stop the service, add and sync
the new user(s) and then restart the service. This ensures multiple processes
are not trying to sync data at the same time, and allows your to monitor the
initial sync for any errors. 


