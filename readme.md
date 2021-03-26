# git-cz

## Description

A rust based cli git helper based on (the javascript version `git-cz`)[https://www.npmjs.com/package/git-cz].

I have no plans to make this extensible.

## Usage

Run: `git-cz`

### Commit message format

```
<type>: <subject>
```

<!--
```
<type>[(<scope>)]: <emoji> <subject>
[BLANK LINE]
[body]
[BLANK LINE]
[breaking changes]
[BLANK LINE]
[footer]
``` -->

The header is the only mandatory part of the commit message.
