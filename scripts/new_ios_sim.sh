#!/bin/bash
ID=$(xcrun simctl create test-iphone com.apple.CoreSimulator.SimDeviceType.iPhone-14-Pro com.apple.CoreSimulator.SimRuntime.iOS-16-1 2>/dev/null)
xcrun simctl boot $ID
xcrun simctl bootstatus $ID
echo Simulator ID is $ID
( trap exit SIGINT ; read -r -d '' _ </dev/tty ) ## wait for Ctrl-C
xcrun simctl delete $ID

    