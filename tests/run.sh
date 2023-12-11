
tests_base_dir=$(dirname $(realpath ${PWD}/${BASH_SOURCE}))

declare -A cmd=( ["part1"]="${CMD}" )
if [ "$CMD2" != "" ]; then
   cmd["part2"]="$CMD2"
else
   cmd["part2"]="$CMD"
fi

run_day_tests() {
   local tests_dir=${tests_base_dir}/data/$1
   local index=${tests_dir}/tests.json

   for file in $(jq -r '. | keys_unsorted[]' ${index}); do
      for part in $(jq -r ".\"${file}\" | keys[]" ${index}); do
         local answer=$(jq -r ".\"${file}\".${part}" ${index})

         echo -n "Testing ${part} ${file}... "
         result=$(${cmd[$part]} ${tests_dir}/${file})
         if [ "$result" = "${answer}" ]; then
            echo "passed"
         else
            echo "failed, $result != ${answer}"
         fi
      done
   done
}