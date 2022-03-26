# focus-ami

A command line tool to kill processes.

Its purpose is to focus for a given period of time, blocking all distracting
apps specified.

## Configuration

The first time running the program, you will be asked to create a configuration
file.

First, you'll be asked to provide an interval between process killing times in
seconds. It's bound in an u8 that corresponds to a bit less than five minutes.
A thirty seconds interval is normally fine.

Then you will be asked for a period of time to focus in minutes. This value is
bound by 65280. This corresponds to 45 days and a bit, but I doubt you
need to focus for that long.

At this point you'll be asked for a "password" this password will be asked
before some actions. The idea being that if the password is long enough
you might think twice before pausing, editing or quitting.

Finally, you will be asked for the list of processes to kill. Because this list
is a list of process names, it can be a little tricky. That's why there is a
helper included with this program, the annotator.

An example of the final configuration file is:

```yaml
---
kill_time: 30
work_time: 30
password: a
processes:
  - firefox-bin
  - vlc.bin
```

It is possible to have a work time that is smaller than the kill time, it makes
sense when you want to run once the killing part, and then quit.

## Annotator

Run the program like this:

`focus -a`

Then start your application. This can fail in three ways.

1. Your user is not the one responsible for the process created.
1. The process was already started.
1. The process is an instance of something else running it, e.g., python3
applications.

The first way can be circumvented by not running the user check using

`focus -A`

If that doesn't work in KDE systems `control-esc` might help you with the
system activity tool. Probably other DEs have some similar tools, such as the
task manager in Windows systems.

In *nix systems other option is to try to identify your process running `ps -e`
while it's running the application to identify.

A better way to do basically the same is to follow this instructions

1. run `$ ps -e > file1`
1. start your application
1. in the same directory as in the first instruction run `$ ps -e > file2`
1. run `$ diff file1 file2`

## Use

### Command Line Arguments

The application accepts the following command line arguments

```
USAGE:
    focus-ami [OPTIONS]

OPTIONS:
    -a, --anotate            Annotator mode, a guide to find processes names
        --anotate-no-user    Annotator mode, a guide to find processes names,
without filtering for
                             user
    -c, --config <CONFIG>    Use the configuration from <CONFIG>. <CONFIG> is a
path starting from
                             your current working directory
    -h, --help               Print help information
    -s, --silent             Disallow interactions during the focus period
    -V, --version            Print version information
```


### Interactions

Possible interactions are:

* `e`: edit the configuration file
* `p`: pause
* `q`: quit early
* `r`: see remaining time
* `a`: add some time to current run (but not to the configuration file)

#### Edit

You can edit parts or your configuration file, while running the program. Your
available options are:

* `k`: edit the time that passes between process killings
* `w`: edit the work time
* `p`: edit the password
* `e`: edit the processes
* `c`: check the config
* `\q`: stop editing
