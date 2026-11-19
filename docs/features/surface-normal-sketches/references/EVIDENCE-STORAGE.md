# Evidence log storage

The `.log` files in this directory tree are readable copies with trailing
whitespace removed and one final newline. Test events, results and measurements
are unchanged.

[The storage manifest](log-storage-2026-11-19.json) maps each changed display copy
to its verbatim member in [the raw archive](raw-evidence-logs-2026-11-19.tar.gz).
It records both hashes and byte counts. Historical receipt hashes describe the
raw capture, so they match the decoded archive member rather than the display
copy. Logs absent from the manifest remain verbatim files.

The archive contains only relative log paths. Every member was compared byte
for byte with its original capture before the display copies were normalized.
The original captures also remain in local checkpoint `6fca95d2`.
