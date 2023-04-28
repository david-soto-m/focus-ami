# focus-ami

A command line tool to focus for a given time. It kills processes given by a
list every so often.

The configuration requires a password. This is not a secret, it's more of an
annoyance so that you don't end up procrastinating or quitting.

From its help:

A command line tool to help focus by killing processes

**Usage**:

`focus-ami [OPTIONS] [FOCUS_PERIOD]`

**Arguments**:

`[FOCUS_PERIOD]`  The time in minutes to focus for

Options:

-   `-c`, `--config`       Edit the configuration
-   `-p`, `--path <PATH>`  Use the configuration at the path
-   `-h`, `--help`         Print help
-   `-V`, `--version`      Print version

## Notes

1. This version **BREAKS BACKWARDS COMPATIBILITY**
    * the config file is not compatible with the configuration of earlier
      versions of the program. It is also called differently.
    * The CLI is different, `annotator` is now imbued inside the `config`
      option which previously was the configuration path but now means that you
      want to edit the config file. It's a mess. If you were relying on a
      previous version to do anything, this version will break everything.
2. This project is in maintenance mode. I have almost finished my master's
  thesis, and I don't think I'll be using it that frequently now. This project
  started as a way to force me to study in the last year of my degree as
  `concentrate` and I could never have predicted what is has become. I can
  almost productively use it now!

  All this to say that I won't be adding new features in the foreseeable future,
  but if you want to contribute or report a bug, please do. I will review your
  contribution or try to solve your bug.

## Installing

### With cargo

`cargo install focus-ami`

### Source install

```
git clone https://github.com/david-soto-m/focus-ami.git
cd focus-ami
cargo install --path .
```
