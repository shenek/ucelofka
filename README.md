# Ucelofka
A program to issue simple invoices via command line.
It is a simple invoice system for the people which are able to use GIT.

Program itself is supposed to operate over a file/directory strucutre
which can be inserted into GIT and maintained there.

The data files should be stored in YAML format.

The default output is supposed to be a html file which can
be used to generate pdf inside your favorite browser.

## Installation

```bash
cargo install --path .
```

## Common workflow

A common procedure how ucelofka should be used.

First we need to create a data directory.

```shell
$ ucelofka project make --target ucelofka-data
```

You can observe the content of the directory afterwards and
edit the yaml files. Afterwards It is a good idea to initialize a GIT repository,
add the files and create initial commit.

```shell
$ cd ucelofka
$ git init .
$ git add *
$ git commit -m "Initial data"
```

Lets create an invoice afterwards.

```shell
$ ucelofka invoice --path . create --account first_account --customer first_customer --entry 001_first_entry --identity first_identity
Created invoice 202000001
```

You can edit the invoice data manually. After that you can simply render the new invoice.

```shell
$ ucelofka invoice render -T default.html -I 202000001
```

Not you can open the generated file in the browser and print it as PDF which can be sent to your customer.
Don't forget to put it in GIT afterwards.

```shell
$ git add invoices/202000001.yml output/202000001.html
$ git commit -m "January 2020"
```

If you want to create another invoice entry for the next month you can simply.
```shell
$ ucelofka entry create --currency USD --id 002_second_entry --name "IT services" --price 2000 --detail "Programming" --detail "Deployment"
```

The rest of the procedure is the same.
```shell
$ ucelofka invoice --path . create --account first_account --customer first_customer --entry 002_second_entry --identity first_identity
$ ucelofka invoice render -T default.html -I 202000002
$ git add invoices/202000002.yml output/202000002.html
$ git commit -m "February 2020"
```

## Default data directory structure

`/accounts`

* Your bank accounts (were money should be sent).

`/customers`

* Data of your customers.

`/entries`

* Items which will be billed to your customer(s).

`/identities`

* Your billing info.

`/invoices`

* Issued invoices.

`/output`

* Rendered invoices.

`/templates`

* Templates which should be used for rendering the invoice.
