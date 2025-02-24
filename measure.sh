#!/usr/bin/env bash

set -euo pipefail

cargo build --release --quiet

duration="${1:-60}"
measurement_id="$(date --rfc-3339=seconds)"

echo "Measurement ID: $measurement_id"
echo "Duration: $duration seconds"

valgrind --tool=massif --massif-out-file="measurements/massif.out.$measurement_id.require" \
  target/release/sqlx-ssl-testing --pg-ssl-mode require --duration-secs "$duration" &
sleep 1
valgrind --tool=massif --massif-out-file="measurements/massif.out.$measurement_id.disable" \
  target/release/sqlx-ssl-testing --pg-ssl-mode disable --duration-secs "$duration" &

wait
