#!/bin/sh
# Isolate desktop unit tests like a process-isolating test runner; do not skip or retry.
# UI and integration harnesses retain their normal scheduling.
case "$1" in
  */deps/photocraft-*)
    exec python3 "$(dirname "$0")/oct22-test-runner.py" "$@"
    ;;
esac
exec "$@"
