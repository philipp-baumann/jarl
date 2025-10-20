# Configuring Jarl

To ensure all uses of Jarl in a project are consistent, it is possible to store options in `jarl.toml`.

For now, this only supports two fields: `select` and `ignore` to determine which rules to use.
This file looks like this:

```toml
[lint]
select = []
ignore = []
```

This has the same capabilities as `--select-rules` and `--ignore-rules`, so it is possible to pass rule names and names of groups of rules:

```toml
[lint]
select = ["PERF", "length_test"]
ignore = ["SUSP"]
```

::: {.callout-note}
## Using CLI arguments and `jarl.toml`

Arguments in the command line always have the priority on those specified in `jarl.toml`.
For example, if you have the following file:
```toml
[lint]
select = ["PERF", "length_test"]
ignore = []
```
then calling
```sh
jarl check . --ignore-rules PERF
```

will only apply the rule `length_test`.
:::
