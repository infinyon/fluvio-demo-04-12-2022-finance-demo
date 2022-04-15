# Fluvio Finance Demo

* This is the source code to the [Real time Event Streaming and Data
Transformation for Financial Services
Webinar](https://www.youtube.com/watch?v=wAvyB8367g4).

## Overview
* Ingest realtime data from https://finnhub.io/docs/api/quote using a Fluvio Connector
* Ingest some local warrants we have
* Use a smartmodule to calculate realtime ROI
* View the data in a Tabular Form

References:
* https://stockmarketmba.com/whatisawarrant.php

# Running the demo

## Setup cluster

The demo can run in [InfinyOn Cloud](https://infinyon.cloud) or your own local cluster.

* **Option 1** (Preferred): Signup for [InfinyOn Cloud](https://www.fluvio.io/docs/get-started/cloud/)
* **Option 2**: Provision your local cluster [Local Cluster](https://www.fluvio.io/download/)
    * Use `fluvio cluster start`, then follow instructions on [Managed Connectors](https://www.fluvio.io/connectors/#managed-connectors)

## Start the connector:

Before starting ensure you have a token from https://finnhub.io and edit the
`quote-data-input.yaml` line to have `token=<YOUR TOKEN>` in the `endpoint`
section of the yaml.

* `make gme-input` - will compile and start the connector
* `fluvio connector create --config ./quote-data-input.yaml`

## View the table
* `make sm-consume-table` will compile the smartstream, create the table and run:
 `fluvio consume --aggregate ./target/wasm32-unknown-unknown/release/finnhub_data_smartmodule.wasm gme-stocks -B 0 --output full-table --table-format profitviews`

## View the raw data
* `make sm-consume` will compile the smartstream and view the raw data via `fluvio consume --aggregate ./target/wasm32-unknown-unknown/release/finnhub_data_smartmodule.wasm gme-stocks -B 0`
