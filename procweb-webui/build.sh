#!/bin/bash

export NG_CLI_ANALYTICS=false
npm i
ng analytics off
ng build --output-hashing none
