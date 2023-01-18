HyperDB CLI
===========

Author: **[Afaan Bilal](https://afaan.dev)**

## Introduction
**HyperDB CLI** is a command line interface (CLI) for the HyperDB Server.

---
## Build and Run
`$ cargo run`

````
Connected to http://localhost:8765
> ?
[HyperDB Client]
COMMANDS:
HELP | ?            Print this help message
PING                Ping server
VERSION | VER       Get server version
HAS [key]           Check if there is a value for [key]
GET [key]           Get the value for [key]
SET [key] [value]   Set the [value] for [key]
DEL [key]           Delete the value for [key]
EMPTY               Check if the store is empty
ALL                 Get all stored data
CLEAR               Delete all stored data from memory
SAVE                Save stored data to disk
RELOAD              Reload store from disk
RESET               Delete all stored data from memory and disk
QUIT | EXIT         Quit
>
````
---

## Contributing
All contributions are welcome. Please create an issue first for any feature request
or bug. Then fork the repository, create a branch and make any changes to fix the bug
or add the feature and create a pull request. That's it!
Thanks!

---

## License
**HyperDB CLI** is released under the MIT License.
Check out the full license [here](LICENSE).
