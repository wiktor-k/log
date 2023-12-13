# Work Log Parser

Parsers work log files which contain dates (`yyyy-dd-mm`) and hours (`nh`) as well as tags (`#tag`) and free-form descriptions.

## Example

This list is a work log:

- 2023-12-13: #log 3h Add README for log project
- 2023-12-14:
  - #log 1h Add license file
  - #other 2h Some other task
- 2023-12-15 4h #other: Yet another task

Now parse the file with:

```sh
cat README.md | cargo run | jq
```

The output of this tool will be a JSONL formatted list of work logs:

```json
{
  "date": "2023-12-13",
  "month": "2023-12",
  "hours": 3,
  "tag": "log",
  "description": "Add README for log project"
}
{
  "date": "2023-12-14",
  "month": "2023-12",
  "hours": 1,
  "tag": "log",
  "description": "Add license file"
}
{
  "date": "2023-12-14",
  "month": "2023-12",
  "hours": 2,
  "tag": "other",
  "description": "Some other task"
}
{
  "date": "2023-12-15",
  "month": "2023-12",
  "hours": 4,
  "tag": "other",
  "description": "Now parse the file with"
}
```

The exact format is subject to change.

The JSON lines can be further processed by `jq`.
