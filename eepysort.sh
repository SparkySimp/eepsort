#!/usr/bin/env bash
# eepy sort - sort by eeping

function worker {
	sleep $1
	echo $1
}

for i in $@; do
	worker $i &
done

wait
