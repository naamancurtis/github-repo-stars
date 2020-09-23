# GitHub Stars CLI App

A simple CLI app for querying the GitHub API based off a username to receive a
list of the top 10 repositories owned by that user, sorted in descending order
according to the number of stars that repo has received.

By default, the output is sent to the terminal, however by providing the
`--path` and `--format` flags you can write the results to either a `JSON` or
`toml` file.

## Example

```sh
ghstar -- naamancurtis -p documents/naamans-repos -f toml

# will output a file to: documents/naamans-repos.toml
```
