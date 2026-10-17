"""Checks for the captured-measurement comparator; no processes are launched."""
import contextlib
import importlib.util
import io
from pathlib import Path
import unittest

spec = importlib.util.spec_from_file_location(
    "launcher_cost_check", Path(__file__).with_name("launcher-cost-check.py")
)
comparator = importlib.util.module_from_spec(spec)
spec.loader.exec_module(comparator)


def report(candidate: list[int]) -> dict[str, dict[str, dict[str, list[int]]]]:
    return {"samples": {"baseline": {"32": [100, 100, 100]}, "candidate": {"32": candidate}}}


class CapturedCostComparison(unittest.TestCase):
    def test_regression_is_red(self) -> None:
        with contextlib.redirect_stdout(io.StringIO()):
            self.assertEqual(comparator.compare(report([200, 200, 200])), 1)

    def test_equal_cost_is_green(self) -> None:
        with contextlib.redirect_stdout(io.StringIO()):
            self.assertEqual(comparator.compare(report([100, 100, 100])), 0)

    def test_bad_measurements_are_rejected(self) -> None:
        for values in ([True, 100, 100], [0, 100, 100], [100], [100, 100, 100, 100]):
            with self.subTest(values=values), self.assertRaises(ValueError):
                comparator.compare(report(values))


if __name__ == "__main__":
    unittest.main()
