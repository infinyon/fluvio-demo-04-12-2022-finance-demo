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

## Start the connector:
* `make gme-input` - will compile and start the connector
* `fluvio connector create --config ./quote-data-input.yaml`

## View the table
* `make sm-consume-table` will compile the smartstream, create the table and run:
 `fluvio consume --aggregate ./target/wasm32-unknown-unknown/release/finnhub_data_smartmodule.wasm gme-stocks -B 0 --output full_table --table-format profitviews`

## View the raw data
* `make sm-consume` will compile the smartstream and view the raw data via `fluvio consume --aggregate ./target/wasm32-unknown-unknown/release/finnhub_data_smartmodule.wasm gme-stocks -B 0`
