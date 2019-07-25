| WARNING: This project is for storing test keys only since they are saved and send plain text. |
| --- |

# Golem Granary

The golem granary is an application to store keys and re-use keys for tests.
This application will be installed on a server accessible by all test-agents

```
golem-granary get_used_account
```
will result in:
```
<KEY_DATA>\n
<TRANSACTION_DATA>
```
when there is an account available to be re-get_used_account
When no account is available the stdout will be empty and exit code 1 will be returned.

```
golem-granary return_used_account --pub "<PUBLIC_KEY_DATA>" --priv "<KEY_DATA>" --transactions "<TRANSACTION_DATA>"
```
will result in:
```
OK
```

re-try please when you get an error
more errors TBD

logs can be found in `~/.granary/logs/granary_<DATE>.log`


## Running from sources on Ubuntu Linux

### Prerequisites

If you don't have a Rust compiler installed, start with:

`sudo apt install rustc cargo`

(or use some other, distribution-dependent
command that installs the rust compiler on your machine)

### Installation

To install `golem-granary` from sources:

1. `git clone https://github.com/golemfactory/golem-granary.git`
2. `cd golem-granary`
3. `cargo build`
4. `sudo cp target/debug/golem-granary /usr/local/bin`
(or otherwise add the `golem-granary` binary to your `PATH`)

done! :)
