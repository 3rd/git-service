# git-service

| ⚠️ WARNING: Do not expose this to The Internet, unless you're ok with people rooting your box. |
| ---------------------------------------------------------------------------------------------- |


> Warning #2: I'm **(very)** new to Rust, and using this for any serious use case is probably not a good idea. But yes, I use it.

## Description

This is a web service that runs on top of a git repository and exposes routes that fetch information from the repository / perform actions.

All the responses are in JSON.

Routes:

- `/remotes` lists remotes
- `/branches/local` lists local branches
- `/branches/remote` lists remote branches
- `/branch/:name` outputs the last commit id for this branch (for now)
- `/branch/:name/checkout` performs a checkout
- `/branch/:name/fetch` performs a fetch
- `/branch/:name/pull` performs a pull

## Why

This is more of a skeleton, which I use as a starting point for some more useful
services. I might merge some more features to it, like routes for git
information (last \$X merges and contributors, activity charts, etc).

It's probably not much use to you, and you could do with a cron script.

## Configuration

All the configuration must be provided via **ENV**.

Sample:

```
GIT_SERVICE_REPO_URL="https://github.com/zurp/repo.git"
GIT_SERVICE_CLONE_PATH="/tmp/repo"
GIT_SERVICE_PORT=3000
```

## Notes

- if the repository does not exits at the specified `$GIT_SERVICE_CLONE_PATH`, it will be
  cloned
- a clone/pull will always happen on start, if neither are successful the service **will crash**
- to debug, launch with `DEBUG=level`, where `level` is one of `debug`,
  `trace`, `info`, `error`
