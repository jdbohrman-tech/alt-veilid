#!/bin/bash

cd ~

rm -rf /srv/*

rm -rf ~/srv

tar -xf repo.tar

cp -R ~/srv/* /srv

#chown -R www-data:www-data /srv