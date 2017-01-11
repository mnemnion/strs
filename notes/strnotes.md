# Implementation notes for strs

## Unit tests

Acccording to (this thread)[https://www.reddit.com/r/rust/comments/3e6zpq/anyone_doing_functional_tests_for_cli_apps_in_rust/] we can mock up command line arguments using `get_matches_with()` in clap and write the normal kind of unit tests. Useful!

## Path forward

    - Turn `strs` into a `cat` clone
    - Build up the configuration structure of the option flags
    - Add string parsing
    - Profit.

## Optional enhancements

    For extra credit, add language-specific parsers. So python strings, rust strings, lua strings... perl strings? yyyyeah unlikely. 