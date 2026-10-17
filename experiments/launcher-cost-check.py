"""Compare captured launcher measurements without executing benchmark programs.

Usage: python3 experiments/launcher-cost-check.py MEASUREMENTS.json
The 20% threshold flags a scheduling-cost symptom, not a CI acceptance policy.
Input follows docs/features/safe-toolkits/scoped-worker-cost-repeat.json.
"""
import json
import statistics
import sys
from pathlib import Path
from typing import TypedDict


class MeasurementReport(TypedDict):
    samples: dict[str, dict[str, list[int]]]


def participant_samples(report: MeasurementReport, implementation: str) -> list[int]:
    values = report["samples"][implementation]["32"]
    if not isinstance(values, list) or len(values) < 3:
        raise ValueError(f"expected at least three samples for {implementation}: {values!r}")
    if any(type(value) is not int or value <= 0 for value in values):
        raise ValueError(f"expected positive integer nanoseconds for {implementation}: {values!r}")
    return values


def compare(report: MeasurementReport) -> int:
    baseline = participant_samples(report, "baseline")
    candidate = participant_samples(report, "candidate")
    if len(baseline) != len(candidate):
        raise ValueError(f"expected paired sample counts: {len(baseline)} != {len(candidate)}")
    old = statistics.median(baseline)
    new = statistics.median(candidate)
    print(f"participants=32 baseline_median_ns={old} candidate_median_ns={new} ratio={new / old:.3f}")
    return int(new > old * 1.2)


def main() -> int:
    [measurement_path] = sys.argv[1:]
    return compare(json.loads(Path(measurement_path).read_text(encoding="utf-8")))


if __name__ == "__main__":
    sys.exit(main())
