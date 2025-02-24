#!/usr/bin/env bash

set -euo pipefail

cargo build --release --quiet

duration="${1:-60}"
measurement_id="$(date --rfc-3339=seconds)"

echo "Measurement ID: $measurement_id"
echo "Duration: $duration seconds"

for ssl_mode in require disable
do
	valgrind \
		--tool=massif \
		--massif-out-file="measurements/massif.out.$measurement_id.$ssl_mode" \
		--time-unit=ms \
		target/release/sqlx-ssl-testing \
		--pg-ssl-mode "$ssl_mode" \
		--duration-secs "$duration" &
	sleep 1
done

wait
