# klib
The kindle notebook export tool.

## Usage

supports two usages：

- export to a directory (each ebook marked with a file).
- sync Github repository.

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
    path      Output to the specified directory.
```

### Subcommand

#### Export to directory

use **path** subcommand

**--outdir**: directory path param, each ebook marked save to independent file, file name use ebook name + .md

```shell
$ ./klib -i ~/Download/MyClippings.txt path --outdir ~/kindle_notes/
```

#### Sync Github

use **github** subcommand

The function use Github API (https://developer.github.com/v3/repos/contents/#create-or-update-a-file) to create and update repo notes. program skip no modify notes.

**--owner**: repository owner name.

**--repo**: repository name.

**--token**: Github OAuth2 token.

```shell
% ./klib -i ~/Desktop/MyClippings.txt github --owner cppcoffee --repo kindle_bookmark --token xxxxxxxxxx
```

The Github sync notes example: https://github.com/cppcoffee/kindle_bookmark

