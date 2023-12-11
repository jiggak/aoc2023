# Tests

The AOC organizers request that we kindly **do not** include puzzle inputs
and sample data in our repos.

Format of the data consumed by the `test.sh` scripts is as follows:

```json
// tests/data/[dayX]/tests.json
{
   "input.txt":
   {
      "part1": "part_one_answer",
      "part2": "part_two_answer"
   },
   "sample.txt":
   {
      "part1": "part_one_sample_answer"
   }
}
```

Input/sample data files are placed in the same directory as `tests.json`.
