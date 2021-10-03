# FSGC - Filesystem Garbage Collector
FSGC cleans directories according to some rules you provide.

# Use Case
Want to clean some directory periodically but you want to be smart about it?
FSGC is made to solve that problem.

FSGC itself is not a daemon or anything, it's just one executable with no dependencies; you
are expected to use it from a task scheduler like `cron` or, on Windows, the `Task Scheduler`.

# How to Install
Grab a binary for your platform on the [releases page](https://github.com/insomnimus/fsgc/releases)
or build it yourself.

# Building From Source
You need the typical rust language setup, read [here](https://www.rust-lang.org/tools/install) for more.

Once that's ready, gaze upon the following lines:

```sh
git clone https://github.com/insomnimus/fsgc
cd fsgc
git checkout main # sometimes required
cargo install --locked --path .
# If above does not work, please add `--target <MY_TARGET>` to the command
# You can also use this one-liner:
# cargo install --locked --branch main --git https://github.com/insomnimus/fsgc
```

# Usage
The usage is very simple:
`fsgc [path/to/config]`

You don't have to specify the config file location from the command line, you can just set `FSGC_CONFIG_PATH`
env variable to the full path of your config file.


If you don't specify a config file either from the command line or from your env, these locations will be assumed:

-	On Windows: `C:\programdata\fsgc.toml`
-	Any OS but Windows: `/etc/fsgc/fsgc.toml`

Now, invoking `fsgc` will evaluate your config file and delete files that need deleting.

# Configuration File and Format
FSGC uses the `TOML` format for its configuration.

## Configuration Sections (Tables)

### The `[options]` Section
This table is used for general options and all keys are optional:

-	`log-file`: String; A Full path to a file for fsgc logs.
If set, every logging goes to this file and not the standard error stream.
-	`header`: String; The banner printed before each invocation of FSGC.
This field supports date and time specifiers, more on that below.
-	`èrror-prefix`: String; A string that will be printed in front of any error in the logs.
This field also supports date and time format specifiers.
-	`òverwrite-logs`: Bool; If set to `true` and there's a log file, the file will be overwritten each time FSGC is invoked.

### The `[rules]` Section
This table is where you configure files/ directories and how they should be handled.
Each entry in this table is in the form

`"path" = <rule>`

Paths may contain spaces or otherwise illegal characters, but make sure to quote them.
Paths may also contain glob patterns in the style of `bash`.

The rule can be specified in two ways:

-	Detailed: `"path" = { age = "duration", created = true, modified = true, accessed = false }`.
Anything but the `age` can be omitted, in which case the defaults are as shown above.
-	Simple: `"path" = "duration".
This is equivalent to `"path" = { age = "duration", created = true, modified = true, accessed = false }`.

## Example Config File

```toml
[options]
log-file = 'D:\home\.config\fsgc\fsgc.log'
header = "# RUN %B %d - %H:%M"
error-prefix = "- "
overwrite-logs = false

[rules]

'D:\home\downloads\*' = "1d" # Delete files older than 1 day.
'D:\home\tmp\*' = { age = "2d", accessed = true } # Delete files older than 2 days but also consider access times to refresh the age.
```

## Duration Format
The `age` field in the `rule` object is a string with the following format:

`<number> <unit>`

The whitespace are ignored and more than one duration can be specified, in which case the values add up:

-	`"1d"`: 1 day.
-	`"1w 3d 5m"`: 1 week, 3 days and 1 minute.
-	`"2 days 12 hours"`: 2 days and 12 hours.

### Duration Units

-	`"ns" | "nanos" | "nanoseconds" | "nanosecond"`: Nanoseconds.
-	`"µs" | "micros" | "microseconds" | "microsecond"`: Microseconds.
-	`"ms" | "millis" | "milliseconds" | "millisecond"`: Milliseconds.
-	`"s" | "sec" | "secs" | "seconds" | "second"`: Seconds.
-	`"m" | "min" | "minute" | "minutes" | "mins"`: Minutes.
-	`"h" | "hr" | "hours" | "hour"`: Hours.
-	`"d" | "day" | "days"`: Days.
-	`"w" | "week" | "weeks"`: Weeks.
-	`"y" | "yr" | "years" | "year"`: Years.

Units are case insensitive.

## Time Format Specifiers
You can use GNU-style time format specifiers in your `header` or `error-prefix`.

Please visit [this page](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html#specifiers) for the full reference.
