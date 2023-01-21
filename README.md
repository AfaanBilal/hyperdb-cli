HyperDB CLI
===========

Author: **[Afaan Bilal](https://afaan.dev)**

## Introduction <img src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/bash/bash-original.svg" alt="Bash" title="Bash" width="50px" style="float: right;" />
**HyperDB CLI** is a command line interface (CLI) for the HyperDB Server.

---
## Run with Docker <img src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/docker/docker-original.svg" alt="Docker" title="Docker" width="50px" style="float: right;" />
`$ docker run --rm -it afaanbilal/hyperdb-cli -a http://host.docker.internal:8765`

````
Connected to http://host.docker.internal:8765
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

## Build and Run <img src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/bash/bash-original.svg" alt="Bash" title="Bash" width="50px" style="float: right;" />
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
