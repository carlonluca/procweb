#!/bin/bash

export NG_CLI_ANALYTICS=false
npm config set allow-git all
npm i
npx ng analytics off
npx ng build --output-hashing none
