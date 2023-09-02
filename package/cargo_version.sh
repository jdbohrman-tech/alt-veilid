#!/bin/bash

INPUTFILE=$1
cat $1 | grep -w version | head -n 1 | cut -d\" -f 2
