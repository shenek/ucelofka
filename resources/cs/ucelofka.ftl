is-not-a-directory = `{ $path }` není adresář
path-not-exits = cesta `{ $path }` neexistuje
data-directory-is-missing-subdir = adresáři s daty `{ $dir_path }` chybí podadresář `{ $subdir_path }`
data-directory-path = cesta k adresáři s daty
response-msg =
    { $value ->
        [one] "{ $input }" has one Collatz step.
       *[other] "{ $input }" has { $value } Collatz steps.
    }
