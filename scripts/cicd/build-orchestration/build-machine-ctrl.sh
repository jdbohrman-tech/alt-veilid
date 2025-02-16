#!/bin/bash

if [ "$1" = "create" ] && [ "$2" = "amd64-deb" ]
then
	## Create amd64-deb build machine
	echo "Creating amd64-deb build machine"
	doctl compute droplet create build-server-amd64-deb-tmp --image 178721112 \
		--size c2-8vcpu-16gb-intel --region nyc1 --enable-private-networking \
		--ssh-keys 38852180,38632397,41187560 --tag-names build-machines,build-orchestration --wait
	echo "Done"

elif [ "$1" = "create" ] && [ "$2" = "arm64-deb" ]
then
	## Create arm64-deb build machine
	echo "Creating arm64-deb build machine"
	doctl compute droplet create build-server-arm64-deb-tmp --image 178721107 \
		--size c2-8vcpu-16gb-intel --region nyc1 --enable-private-networking \
		--ssh-keys 38852180,38632397,41187560 --tag-names build-machines,build-orchestration --wait
	echo "Done"

elif [ "$1" = "create" ] && [ "$2" = "amd64-rpm" ]
then
	## Create amd64-rpm build machine
	echo "Creating amd64-rpm build machine"
	doctl compute droplet create build-server-amd64-rpm-tmp --image 178721114 \
		--size c2-8vcpu-16gb-intel --region nyc1 --enable-private-networking \
		--ssh-keys 38852180,38632397,41187560 --tag-names build-machines,build-orchestration --wait
	echo "Done"

elif [ "$1" = "create" ] && [ "$2" = "arm64-rpm" ] ## This snapshot does not yet exist
then
	## Create arm64-rpm build machine
	echo "Creating arm64-rpm build machine"
	doctl compute droplet create build-server-arm64-rpm-tmp --image 178721114 \
		--size c2-8vcpu-16gb-intel --region nyc1 --enable-private-networking \
		--ssh-keys 38852180,38632397,41187560 --tag-names build-machines,build-orchestration --wait
	echo "Done"

elif [ "$1" = "delete" ] && [ "$2" = "amd64-deb" ]
then
	## Delete amd64-deb build machine
	echo "Deleting amd64-deb build machine"
	doctl compute droplet delete build-server-amd64-deb-tmp --force
	echo "Done"

elif [ "$1" = "delete" ] && [ "$2" = "arm64-deb" ]
then
	## Delete arm64-deb build machine
    echo "Deleting arm64-deb build machine"
    doctl compute droplet delete build-server-arm64-deb-tmp --force
    echo "Done"

elif [ "$1" = "delete" ] && [ "$2" = "amd64-rpm" ]
then
	## Delete amd64-rpm build machine
    echo "Deleting amd64-rpm build machine"
    doctl compute droplet delete build-server-amd64-rpm-tmp --force
    echo "Done"

elif [ "$1" = "delete" ] && [ "$2" = "arm64-rpm" ] ## This snapshot does not exist yet
then
	## Delete arm64-rpm build machine
    echo "Deleting arm64-rpm build machine"
    doctl compute droplet delete build-server-arm64-rpm-tmp --force
    echo "Done"
else
	echo $1 "is not a valid command to execute for "$2
fi