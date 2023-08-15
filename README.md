# NATS-Ractor Demo

This repository contains three packages to demonstrate the use of a [Ractor](https://slawlor.github.io/ractor/) as a framework to invoke actors:

* `nr-ractor` - a Ractor cluster node that hosts actors.
* `nr-actors` - a shared library that defines actors.
* `nr-nats` - a NATS-based microservice that is also a Ractor cluster node.

## Prerequisites

You will need a locally-running NATS server. Either use a docker container or install and run `nats-server`. (See [Installing a NATS Server](https://docs.nats.io/running-a-nats-service/introduction/installation) for details).

You will also need to install the NATS CLI: (See [Installing the NATS CLI Tool](https://docs.nats.io/running-a-nats-service/clients#installing-the-nats-cli-tool))

## Running the Demo

Open 4 terminal windows:

1. In the first window, start the NATS server:

    ```bash
    $ nats-server
    ```

2. In the second window, start the Ractor cluster node:

    ```bash
    $ make run-ractor
    ```

3. In the third window, start the NATS-based microservice:

    ```bash
    $ make run-nats
    ```

4. In the fourth window, make a NATS request:

    ```bash
    $ nats request greetings "{ \"name\": \"Wibble\" }"
    ```

You should see a response message in the NATS CLI window as well as some output in the NATS service window to show some processing details.

