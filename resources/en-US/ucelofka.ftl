is-not-a-directory = `{ $path }` is not a directory
path-not-exits = path `{ $path }` does not exist
data-directory-is-missing-subdir = data directory `{ $dir_path }` is missing `{ $subdir_path }` subdir
data-directory-path = path to data directory
response-msg =
    { $value ->
        [one] "{ $input }" has one Collatz step.
       *[other] "{ $input }" has { $value } Collatz steps.
    }
