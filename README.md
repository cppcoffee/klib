# klib
The kindle notebook export tool.

## usage

supports two usages：

- export to directory (each ebook marked with a file).
- sync github repository.

```shell
% ./klib --help

klib 0.1.0
the kindle notebook export tool.

USAGE:
    klib [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <input>    Input file [default: /Volumes/Kindle/documents/My Clippings.txt]

SUBCOMMANDS:
    github    Upload notes to github repository.
    help      Prints this message or the help of the given subcommand(s)
    path      Output to specified directory.
```

### subcommand

#### export to directory

use **path** subcommand

**--outdir**: directory path param, each ebook marked save to independent file, file name use ebook name + .md

```shell
$ ./klib -i ~/Download/MyClippings.txt path --outdir ~/kindle_notes/
```

#### sync github

use **github** subcommand

The function use github API (https://developer.github.com/v3/repos/contents/#create-or-update-a-file) create and update repo notes. program skip no modify notes.

**--owner**: repository owner name.
**--repo**: repository name.
**--token**: github OAuth2 token.

```shell
% ./klib -i ~/Desktop/MyClippings.txt github --owner cppcoffee --repo kindle_bookmark --token xxxxxxxxxx
```

The github sync notes example: https://github.com/cppcoffee/kindle_bookmark

