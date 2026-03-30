#!/bin/bash

#echo " -  |-\\  -"
#echo "\\   | | \\"
#echo " -  |--  -"
#echo "  / | \\   /"
#echo " -  |  \\ -"
# this is garbage lmao.

echo -e "\x1b[1;34m=== Building surge-rs is a bit intricate. Go get some coffee. ==="
echo -e "\x1b[1;95mWe're moving like this: [SRS-SYS] -> [SRS-BRG] -> [SRS-LIB].\x1b[0m"
cargo build --color always -vv --examples 2>&1 | python3 -u <(cat << "EOF"
import sys, random, re

pattern = re.compile(r'\[surge-rs [0-9.]*\]')
matches = ['[SRS', 'Downloaded', 'Fresh', 'Compiling', 'Building', 'Finished', 'Performing', 'Detecting']
progress = ".:/\\o-^#|\"!$%&=*"

for l in sys.stdin:
    #l = l.lstrip().rstrip().replace('\x1b[A', '')
    #l = l.replace('\x1b[A', '')
    l = re.sub(r"\[surge-.*? \d+.\d+.\d+\]", "", l)
    if "could not compile" in l:
        print('\x1b[1;31m')
        print('#############################')
        print('#YO! ERROR! HEED MY WARNING!#')
        print('#############################')
        print('\x1b[0m\x1b[1m', end='')
        print('There has been an error when building surge-rs.')
        print('This filter obscures thousands of lines of garbage, which is a net positive.')
        print('...It also obscures a good chunk of the error.')
        print()
        print('To debug this error, go ahead and disable this filter. Or give me a call. ', end='')
        print('\x1b[1;31mGood luck! (-:\x1b[1m')
        print('----------------------------------------------------------------------------------------\x1b[0m')
        print(l)
    elif ("error:" in l or "process didn't" in l) and "Running" not in l:
        print('\x1b[1;31m ERR!!! => '+l+'\x1b[0m')
    elif any(m in l for m in matches):
        print("\r\x1b[0;34m[OK!]\x1b[0m"+pattern.sub('', l), end='')
    #elif 'Running' in l:
    else:
        print('\r\x1b[0;95m[' + ''.join(random.choices(progress, k=3)) + ']\x1b[0m', end='')
EOF
)
