# p4mate

## Background

p4mate is a tool that helps the game developers who use Perforce to streamline their workflow.

it's written in Rust, built in using Clap v4.

it offers a git-like interface with several commands.

it works on Windows, Linux and MacOS.

## Commands

### p4mate unlock

`p4mate unlock <dir> ...` to lift the read-only lock on the given directories or files

`p4mate unlock <dir> ...` should have the same behavior as such:

On Windows, it is equivalent to a user opening file explorer, navigating to the directory, right
clicking, selecting `properties` and unchecking the `read-only` option.

on Linux and MacOS, it should remove the `ro` bit from the directories and files but keep other bits
unchanged.
