#!/bin/bash

pushd "$(dirname $0)" > /dev/null

nvim-qt -- -c 'e src/main.rs' &
