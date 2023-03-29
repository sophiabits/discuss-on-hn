# discuss-on-hn

## Overview

A simple serverless function that, given a URL, will return a link to its Hacker News submission if one exists. I use this to generate "Discuss on HN" links for [sophiabits.com/blog](https://sophiabits.com/blog).

## Getting started

Make sure you have [cargo-lambda](https://github.com/cargo-lambda/cargo-lambda) installed.

**Running locally**

```sh
$ cargo lambda watch
$ cargo lambda invoke --data-ascii "{\"url\":\"https://google.com/io\"}"
{"ok":true,"url":"https://news.ycombinator.com/item?id=14360073"}
```

**Deploying to AWS**

Assuming you have an `AWS_ACCESS_KEY` and `AWS_SECRET_ACCESS_KEY` set in your environment:

```sh
$ cargo lambda build --release --arm64
$ cargo lambda deploy
```
