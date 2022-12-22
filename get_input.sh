#!/bin/bash

args=("$@")
cookies="${PWD}/cookies.txt"

if (( $# != 1 ))
then
	echo "Please pass exactly one argument: day number (1-25)"
	exit 3
fi

if (( ${args[0]} < 1 )) || (( ${args[0]} > 25 ))
then
	echo "Please pass a number within range: day number (1-25)"
	exit 4
fi

output_dir="${PWD}/day_${args[0]}"

if [ ! -f $cookies ]
then
	echo "cookies.txt does not exist in current directory"
	exit 5
fi

if [ ! -d $output_dir ]
then
	echo "Directory $output_dir does not exist"
	exit 6
fi

wget --no-cookies --header \
	"Cookie: session=$(<$cookies)" \
	-O "$output_dir/input.txt" \
	"https://adventofcode.com/2022/day/${args[0]}/input"
