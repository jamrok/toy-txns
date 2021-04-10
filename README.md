Toy Transactions
---

This is a simple payment processing application. It takes a list of client transactions in a CSV file, processes them and then prints a summary of the clients' account on the screen. It can currently handle `deposit`, `withdrawal`, `dispute`, `resolve` and `chargeback` transactions. All other transaction types are ignored.

Requirements
---
On your local machine, please ensure that the following is installed:
- `Make` - [`Mac`](https://formulae.brew.sh/formula/make), [`Linux`](https://snapcraft.io/ubuntu-make)
- `Rust & Cargo` - [`Mac` & `Linux`](https://www.rust-lang.org/tools/install)


Usage
---

Once you have the required tools, you can do the following:

Run `make` to get help on what commands are available.
```
$ make
Run 'make' with one or more of the following targets:
 help               - Show a list of all the available make targets
 release            - Build the release version of the application.
 run                - Run the dev version of the application. Set the $FILE variable to override the default file.
 run_all_targets    - Run all the make targets to see if they are valid (Use with caution).
 run_release        - Run the release version of the application. Set the $FILE variable to override the default file.
 test               - Run the test suite.
```

### How to run the dev version of the application on a specific file
The following shows how to test the program with a particular input file by setting the `FILE` environment variable.

 ```
$ make run FILE=tests/precision.csv
cargo run -- ${FILE:-tests/transactions.csv}
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/toy-txns tests/precision.csv`
client,available,held,total,locked
1,1.4767,0,1.4767,false
2,0.9995,0,0.9995,false
```
The contents of the input file is as follows:
```
$ cat tests/precision.csv
type,        client,  tx,  amount
deposit,     1,       1,   1.01
deposit,     2,       2,   2.023
deposit,     1,       3,   2.0456
withdrawal,  1,       4,   1.57891
withdrawal,  2,       5,   1.023456
```

You can also run the application against a file by running `cargo run` as shown below:
```
$ cargo run -- tests/transactions3.csv
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/toy-txns tests/transactions3.csv`
client,available,held,total,locked
1,1.5,0,1.5,false
2,3,0,3,true
7,0,0,0,false
```

### How to run tests
You can run the test suite by running the following command:
```
$ make test
cargo test -- --nocapture
    Finished test [unoptimized + debuginfo] target(s) in 0.01s
     Running target/debug/deps/toy_txns-982fedc1269e2c75

running 1 test
test tests::precision_64 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running target/debug/deps/toy_txns-030dc813809be40c

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running target/debug/deps/integration-df445cda63deb830

running 2 tests
...snip...
test tests::invalid_file ... ok
...snip...
test tests::sample_transactions ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running target/debug/deps/unit-5d5c74c7315879f9

running 1 test
...snip...
test tests::missing_file ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
...snip...
```
You can also run tests by running `cargo test`.

A lot more unit tests need to be written when time allows. However, the integration test that uses the sample files, tests a few different scenarios.
For example:
- Ensuring precision is at most 4 decimal places.
- Outputting the same level of precision given (with acceptable rounding).
- Transactions out of order are processed correctly.
- Withdrawals fail if there are not enough available funds.
- Disputes only occur if the related transaction exists.
- Both `resolve` and `chargeback` transactions are only applied to disputed transactions.
- Account is locked when a chargeback occurs
- Account is locked when a client disputes a transaction, but doesn't have enough available funds to reconcile it.
- Locked accounts cannot process any new transactions.
- New clients with transactions other than deposits get created with a zero balance.
- Invalid lines in a CSV file are ignored if other lines are parsable.
- Invalid files are not processed.

### How to build the production (release) version of the application:
```
$ make release
cargo build --release
   Compiling toy-txns v0.1.0 (toy-txns)
    Finished release [optimized] target(s) in 0.97s
cp -vf target/release/toy-txns .
'target/release/toy-txns' -> './toy-txns'
```
After running the command above, you should see output similar to that shown above.
Note that the application has also been copied to the local directory, so you are also able to run it directly.


### How to run the release version of the application
After building the application, you can run it directly as shown below:
```
$ ./toy-txns tests/transactions.csv
client,available,held,total,locked
1,1.5,0,1.5,false
2,2,0,2,false
```

Alternatively, you can run it with the following make command:
```
$ make run_release FILE=tests/transactions.csv
client,available,held,total,locked
1,1.5,0,1.5,false
2,2,0,2,false
```

### How to handle errors
The input files must be formatted correctly or you will receive an error message that points you to the section of the file that could not be parsed.

##### eg. Invalid file
```
./toy-txns tests/bad_file.csv
Error: CSV deserialize error: record 3 (line: 4, byte: 99): field 1: invalid digit found in string
```
The above output shows that there was a problem processing the `tests/bad_file.csv` file.

If you examine that file, you'll see that line 4 doesn't have the correct entry for the client column and this caused the file to not be parsed.
```
$ cat tests/bad_file.csv
type,        client,  tx,  amount
deposit,     3,       1,   5.4321
deposit,     2,       2,   2.0
withdrawal,  James Bond, 5,   3.0
dispute,     3,       2,
withdrawal,  3,       2,   2.0
```
##### eg. Invalid line in file
```
$ ./toy-txns tests/bad_line.csv
Unexpected Transaction type: TransactionDetails { action: "cashback", client: 3, id: 5, amount: Some(10.0) }
client,available,held,total,locked
2,2,0,2,false
3,3.4321,0,3.4321,false
```

The above output shows that there was a problem processing one of the lines in the `tests/bad_line.csv` file.

If you examine that file, you'll see that line 3 doesn't have a valid transaction type. That transaction was ignored and the rest of the transactions were processed.
```
$ cat tests/bad_line.csv
type,        client,  tx,  amount
deposit,     3,       1,   5.4321
cashback,    3,       5,   10.0
deposit,     2,       2,   2.0
dispute,     3,       2,
withdrawal,  3,       2,   2.0
```


---
![The End...or is it?](https://media.giphy.com/media/rPuuCF2SCiRl6/giphy.gif)
