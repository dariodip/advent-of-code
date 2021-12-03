#!/bin/bash 

usage() {
    echo "Usage: $0 <year> <day|start_day,end_day> <output_dir>"
    echo "Example: $0 2019 1  # downloads input file only for the first day"
    echo "Example: $0 2020 1,10 # downloads input files for every day from 1 to 10"
    exit $1
}

if [ $# -lt 3 ]; then
    usage 1
fi

# show help
if [ "$1" == "-h" ]; then
    usage 0
fi

for day in $(echo $2 | tr "," "\n"); do
    echo "Downloading day $day"
    curl -s -o "$3/day_$day.txt" "https://adventofcode.com/$1/day/$day/input"
done
