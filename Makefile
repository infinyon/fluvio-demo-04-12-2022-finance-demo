

sm-compile:
	cargo build   --release --target wasm32-unknown-unknown

sm-upload: sm-compile
	fluvio smart-module create --wasm-file ./target/wasm32-unknown-unknown/release/finnhub_data_smartmodule.wasm price-warrant-aggregator

sm-consume: sm-compile
	fluvio consume --aggregate ./target/wasm32-unknown-unknown/release/finnhub_data_smartmodule.wasm gme-stocks -B 0

sm-consume-table : sm-compile table-format
	fluvio consume --aggregate ./target/wasm32-unknown-unknown/release/finnhub_data_smartmodule.wasm gme-stocks -B 0 --output full-table --table-format profitviews

produce-warrants:
	cat warrants.txt  | fluvio produce gme-stocks

gme-input: sm-upload
	fluvio connector create --config ./quote-data-input.yaml

table-format:
	fluvio tf delete profitviews || true
	fluvio table-format create --config ./table-view.yaml

clean:
	fluvio connector delete gme-http-input || true
	fluvio tf delete profitviews || true
	fluvio sm delete price-warrant-aggregator || true
	fluvio topic delete gme-stocks || true
