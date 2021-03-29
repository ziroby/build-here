# Build Here (`bh`) &mdash; A universal build tool

The `bh` (Build Here) program is a simple tool which looks at the
current directory and builds it, using the correct tool. This is
especially useful for polyglots who change language from one task to
the next.

Alternative build methods are hard-coded into the source, but that may
change in the future.

## Usage

Run the command with no arguments to build the current directory in
the most common way.

If you pass any command line arguments, they are passed unchanged to
the correct build program.

## Implementation

The proper build tool is determined by looking for an indicator file. For example,
if a file named `Makefile` exsits, it calls `make`.

Build tools are ordered in a logical way. This ensures, for instance,
that if a `gradlew` file and a `build.gradle` file both exist, the
`./gradlew` file is used for building.

## Bugs

There is currently no way to display the list of programs and test files.

New tools must be added to the source to be detected. This may remain
this way forever. I don't really want a generic tool that everyone has
to configure to get their build tool to work. I'd prefer a tool that
"just works" for every conceivable build tool.

The number of tools are very limited currently. This is mainly due the
author not adding them all yet.

There are no tests.

This is all based on the author's opinionated opinion of what tools
exist and what order they should be considered.
