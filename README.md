# pdfzf
_pdfzf_ is a simple fuzzy find tool for PDFs, written in Rust. Essentially, it allows you to grep for search
terms in a large number of PDFs efficiently.

**Features:**

- Fuzzy find implemented using the Skim algorithm (similar to the `fzf` tool)
- Parallelised using Rayon (blazing fast!!!! :rocket: :rocket: :rocket:)

**Usage:**

Using pdfzf is very straightforward: `pdfzf <search_term> <path_to_files>`.

A typical invocation will look like `pdfzf test *.pdf`.

## TODO
- Option to disable ignoring case

## Licence
Copyright (c) 2024 Matt Young.

pdfzf is available under the ISC license.
