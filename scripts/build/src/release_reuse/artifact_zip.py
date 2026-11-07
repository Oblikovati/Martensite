"""Read a bounded, flat artifact without extracting paths onto the filesystem."""
import json
import re
import sys
import zipfile

MAX_BYTES = 1024 * 1024 * 1024
with zipfile.ZipFile(sys.argv[1]) as archive:
    entries = archive.infolist()
    names = [entry.filename for entry in entries]
    if not 0 < len(names) <= 4096 or len(set(names)) != len(names):
        raise ValueError("expected 1..4096 unique artifact entries")
    if any(len(name) > 255 or not re.fullmatch(r"[A-Za-z0-9_][A-Za-z0-9_.-]*", name) for name in names):
        raise ValueError("expected flat artifact filenames")
    if any(entry.is_dir() or entry.flag_bits & 1 for entry in entries):
        raise ValueError("expected unencrypted file entries")
    if sum(entry.file_size for entry in entries) > MAX_BYTES:
        raise ValueError("artifact expanded size exceeds 1 GiB")
    if len(sys.argv) == 2:
        print(json.dumps(names))
    else:
        with archive.open(sys.argv[2]) as member:
            payload = member.read(MAX_BYTES + 1)
            if len(payload) > MAX_BYTES:
                raise ValueError("artifact payload exceeds 1 GiB")
            sys.stdout.buffer.write(payload)
