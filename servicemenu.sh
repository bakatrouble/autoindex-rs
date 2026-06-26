#!/bin/bash

import
URLENCODED_PATH=$(printf %s "$1" | jq -sRr @uri)
if http --check-status --ignore-stdin post "http://localhost:8474/api/chroot?new_root=$URLENCODED_PATH";
then
  RET=$(notify-send --app-name autoindex-rs "Root dir was updated" --action=OPEN=Open -t 5000)
  case $RET in
    "OPEN")
      xdg-open http://localhost:8474
      ;;
  esac
else
  notify-send --app-name autoindex-rs "An error has occurred"
fi
