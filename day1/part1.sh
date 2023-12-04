#!/bin/bash

if [ "$1" = "" ]; then
   echo "$0 [input.txt]"
   exit 1
fi

total=0

while read word; do
   left=""
   right=""
   start=""

   # iterate word chars left to right
   for (( i=0; i<${#word}; i++ )); do
      char="${word:$i:1}"
      if [ "$char" -eq "$char" ] 2>/dev/null; then
         left=$char
         start=$i
         break
      fi
   done

   # iterate word chars right to left
   for (( i=${#word}-1; i>${start}; i-- )); do
      char="${word:$i:1}"
      if [ "$char" -eq "$char" ] 2>/dev/null; then
         right=$char
         break
      fi
   done

   if [ "$left" = "" ]; then
      left=$right
   elif [ "$right" = "" ]; then
      right=$left
   fi


   total=$(( $total + $left$right ))

   if [ "$DEBUG" = "1" ]; then
      echo "$word: $left $right"
   fi

done <$1

echo $total