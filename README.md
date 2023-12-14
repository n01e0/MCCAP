[![CI](https://github.com/n01e0/MCCAP/actions/workflows/CI.yml/badge.svg)](https://github.com/n01e0/MCCAP/actions/workflows/CI.yml)

# MCCAP
Merry Christmas! Chicken Art Protocol

## about
[ref](https://pages.d-sato.net/2023/12/13/post.html)

> All messages are encoded in UTF-8 & send via TCP. No encryption, no compression, no security. If not specified, all messages are ended with ‘\n’ character.
> 
> Request & Response Request to server with ‘Merry’. Server response with ‘Christmas!’. Request to server with ‘Chicken’. Server response with fried chicken art with “C” delimiter. End of protocol, close connection.


## usage

```
$ cargo r &
$     Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/MCCAP`
nc localhost 8080
Merry
Christmas
Chicken

          ████████
        ██      ▒▒██
      ██    ▒▒▒▒▒▒▓▓██
    ██  ░░▒▒▒▒▒▒▒▒▒▒▓▓██
    ██░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒██
  ██  ░░▒▒▒▒▒▒▒▒░░▒▒▒▒▓▓██
  ██░░░░▒▒▒▒▒▒▒▒▒▒░░▒▒▓▓██
  ██░░▒▒▒▒▒▒▒▒▒▒░░░░▒▒▓▓██
    ██▒▒▒▒▒▒▒▒▒▒░░▒▒▒▒██
  ░░██░░▒▒▒▒▒▒▒▒▒▒▒▒▓▓██
    ░░██▒▒▒▒▒▒▒▒▒▒▓▓██
        ██▒▒▒▒▒▒▓▓██
        ██▒▒▒▒▒▒▓▓██
          ████████
          ██░░░░██
          ██░░░░██
          ██  ░░██
        ██    ░░░░██
      ██    ████░░░░██
        ████    ████
```
