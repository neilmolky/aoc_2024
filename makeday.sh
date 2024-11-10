#!/bin/bash
for i in {01..25}
do
    cp src/template.rs src/days/day$i.rs
    touch data/day$i.txt
done