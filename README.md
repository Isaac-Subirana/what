```
           _           _   
          | |         | |  
 __      _| |__   __ _| |_ 
 \ \ /\ / / '_ \ / _` | __|
  \ V  V /| | | | (_| | |_ 
   \_/\_/ |_| |_|\__,_|\__| : an adaptation of Linux's "which" (or Windows' "where"), written in Rust.    
``` 
_<h6> Ascii art generated using https://patorjk.com/software/taag/. </h6>_

This utility searches for all of the occurrences of the passed arguments in your system's PATH (appending all of the PATH extensions if you are on Windows or checking if the found files have execution permissions in Linux).

### Usage:

```shell
what [options] <arguments>
```

### Available options:
  - _`[no options]`_ : shows all the occurrences of the passed arguments found in the system's PATH and a message enumerating them.
  - `-m` _(minimal)_ : shows a minimal, non-colored, _which-like_ output (only one occurrence of each argument and no extra messages).
  - `-s` _(silent)_ : runs silently and returns 0 if one occurrence of every argument is found (and 1 otherwise). Will override `-m` if both are passed.
  - `-h` or `--help` _(help)_ : shows a help menu detailing these options and the usage of the utility. Will override all other arguments passed. 