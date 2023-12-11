#!/bin/bash

CMD="dotnet run --property WarningLevel=0"
CMD2="dotnet run --property WarningLevel=0 -- -p2"
source ../tests/run.sh

run_day_tests day4