<div align="center"><h1>jarl</h1></div>
<div align="center"><i>Just Another R Linter</i> </div>

<br>
`jarl` is a linter for R. It is written in Rust and built on [Air](https://posit-dev.github.io/air/), a fast formatter for R.

## Installation

macOS and Linux:
```
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/etiennebacher/jarl/releases/download/v0.0.17/jarl-installer.sh | sh
```

Windows:

```
powershell -ExecutionPolicy Bypass -c "irm https://github.com/etiennebacher/jarl/releases/download/v0.0.17/jarl-installer.ps1 | iex"
```

Alternatively, if you have Rust installed, you can get the development version with:

```
cargo install --git https://github.com/etiennebacher/jarl
```

## Acknowledgements

* [`lintr` authors and contributors](https://lintr.r-lib.org/authors.html): while the infrastructure is completely different, all the rule definitions and a large part of the tests are inspired or taken from `lintr`.
* Davis Vaughan and Lionel Henry, both for their work on Air and for their advices and answers to my questions during the development of `jarl`.
* R Consortium for funding part of the development of `jarl`.


![](r-consortium-logo.png)
