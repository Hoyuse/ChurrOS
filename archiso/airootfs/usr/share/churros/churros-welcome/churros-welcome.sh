#!/usr/bin/env bash

cd /usr/share/churros/churros-welcome

env > /tmp/churros-env.txt

sleep 3

env >> /tmp/churros-env.txt

awww img /usr/share/churros/wallpapers/default.png --transition-type none \
    > /tmp/awww.log 2>&1

exec /usr/bin/python3 src/main.py