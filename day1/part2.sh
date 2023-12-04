#!/bin/bash

if [ "$1" = "" ]; then
   echo "$0 [input.txt]"
   exit 1
fi

declare -A digit_words=( [one]=1 [two]=2 [three]=3 [four]=4 [five]=5 [six]=6 [seven]=7 [eight]=8 [nine]=9 )

parse_digit() {
   local word=$1
   local i=$2

   local char="${word:$i:1}"
   if [ "$char" -eq "$char" ] 2>/dev/null; then
      echo $char
      return 0
   fi

   for digit_word in "${!digit_words[@]}"; do
      if [[ ${word:$i} == ${digit_word}* ]]; then
         echo ${digit_words[$digit_word]}
         return 0
      fi
   done

   return 1
}

total=0

while read word; do
   left=""
   right=""
   start=""

   # iterate left to right
   for (( i=0; i<${#word}; i++ )); do
      val=$(parse_digit $word $i)
      if [ $? -eq 0 ]; then
         left=$val
         start=$i
         break
      fi
   done

   # iterate right to left
   for (( i=${#word}-1; i>${start}; i-- )); do
      val=$(parse_digit $word $i)
      if [ $? -eq 0 ]; then
         right=$val
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