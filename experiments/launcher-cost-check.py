"""Compare captured launcher measurements without executing benchmark programs.

Usage: python3 experiments/launcher-cost-check.py MEASUREMENTS.json
Rejects any observed median slowdown at any supplied participant count (#87).
Passing captured samples does not establish noise-controlled performance equivalence.
Input follows docs/features/safe-toolkits/scoped-worker-cost-repeat.json.
"""
import json
import statistics
import sys
from pathlib import Path
from typing import TypedDict


class MeasurementReport(TypedDict):
    samples: dict[str, dict[str, list[int]]]


def participant_samples(
    report: MeasurementReport, implementation: str, participants: str
) -> list[int]:
    values = report["samples"][implementation][participants]
    if not isinstance(values, list) or len(values) < 3:
        raise ValueError(f"expected at least three samples for {implementation}: {values!r}")
    if any(type(value) is not int or value <= 0 for value in values):
        raise ValueError(f"expected positive integer nanoseconds for {implementation}: {values!r}")
    return values


def compare_participants(report: MeasurementReport, participants: str) -> int:
    baseline = participant_samples(report, "baseline", participants)
    candidate = participant_samples(report, "candidate", participants)
    if len(baseline) != len(candidate):
        raise ValueError(f"expected paired sample counts: {len(baseline)} != {len(candidate)}")
    old = statistics.median(baseline)
    new = statistics.median(candidate)
    print(f"participants={participants} baseline_median_ns={old} candidate_median_ns={new} ratio={new / old:.3f}")
    return int(new > old)


def compare(report: MeasurementReport) -> int:
    baseline = set(report["samples"]["baseline"])
    candidate = set(report["samples"]["candidate"])
    if not baseline or baseline != candidate:
        raise ValueError(f"expected matching nonempty participant sets: {baseline} != {candidate}")
    if any(not key.isdecimal() or int(key) <= 0 for key in baseline):
        raise ValueError(f"expected positive integer participant labels: {baseline}")
    verdicts = [compare_participants(report, count) for count in sorted(baseline, key=int)]
    return int(any(verdicts))


def main() -> int:
    [measurement_path] = sys.argv[1:]
    return compare(json.loads(Path(measurement_path).read_text(encoding="utf-8")))


if __name__ == "__main__":
    sys.exit(main())
