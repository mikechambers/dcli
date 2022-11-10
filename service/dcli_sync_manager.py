import time
from subprocess import run, PIPE
import datetime

while True:

    p = run( [ 'dclisync', '--sync' ], stdout=PIPE, stderr=PIPE )

    now = datetime.datetime.now()
    if p.returncode == 1:
        print( now.strftime("%Y-%m-%d %H:%M:%S"), '[stdout]', p.stdout.decode() )
        print( now.strftime("%Y-%m-%d %H:%M:%S"), '[stderr]', p.stderr.decode() )
    elif p.returncode == 0:
        print( now.strftime("%Y-%m-%d %H:%M:%S"), '[stdout]', p.stdout.decode() )

    time.sleep(30)