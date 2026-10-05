#!/usr/bin/env bash

# Change to the script's own directory
cd "$(dirname "$0")" || exit 1

echo "Starting updates. Will run every 6 hours. Press Ctrl+C to stop, unless you disowned."

while true; do
    echo "[$(date)] Updating..."
    ./mal-discord-widget
    echo "[$(date)] Finished, sleeping for 6 hours."
    sleep 21600
done
