"""Diagnostic comparison of existing workload binaries; not a CI acceptance gate.

Usage: python3 experiments/launcher-cost-check.py BASELINE CANDIDATE
The 20% threshold flags a repeatable scheduling-cost symptom for investigation.
"""
import re
import statistics
import subprocess
import sys


def measure(binary):
    result = subprocess.run(
        [binary, "persistent_range_workload", "--ignored", "--nocapture"],
        check=True, capture_output=True, text=True, timeout=30,
    )
    values = re.findall(
        r"participants=32 sample=\d+ launches=1000 items=64000 elapsed_ns=(\d+)",
        result.stdout,
    )
    assert len(values) == 3, result.stdout
    return [int(value) for value in values]


def main():
    baseline, candidate = sys.argv[1:]
    samples = {baseline: [], candidate: []}
    for round_index in range(6):
        order = [baseline, candidate] if round_index % 2 == 0 else [candidate, baseline]
        for binary in order:
            samples[binary].extend(measure(binary))
    old = statistics.median(samples[baseline])
    new = statistics.median(samples[candidate])
    print(f"participants=32 baseline_median_ns={old} candidate_median_ns={new} ratio={new / old:.3f}")
    return int(new > old * 1.2)


if __name__ == "__main__":
    sys.exit(main())
