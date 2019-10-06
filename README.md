# git-service

| ⚠️ WARNING: Do not expose this to The Internet, unless you're ok with people rooting your box. |
| --- |

> Warning #2: I'm **(very)** new to Rust, and using this for any serious use case is probably not a good idea. But yes, I use it.

## Description

This service exposes a webhook (`/update`), that will pull the latest changes of a git
repository. It's made to be extended by other routes that operate on the
repository.

## Why

This is more of a skeleton, which I use as a starting point for some more useful
services. I might merge some more features to it, like routes for git
information (last \$X merges and contributors, activity charts, etc).

It's probably not much use to you, and you could do with a cron script.

## Configuration

All the configuration must be provided via **ENV**.

Sample:

```
REMOTE=https://github.com/zurp/repo.git
BRANCH=master
CLONE_PATH=/tmp/repo
PORT=3000
```

## Notes

- if the repository does not exits at the specified `CLONE_PATH`, it will be
  cloned
- a clone/pull will always happen on start
- to debug, launch with `DEBUG=level`, where `level` is one of `debug`,
  `trace`, `info`, `error`

