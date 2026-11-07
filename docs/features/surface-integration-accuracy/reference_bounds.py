"""Exact rational area enclosure, reusing the independently recorded elliptic proof."""
from fractions import Fraction
from decimal import ROUND_CEILING, ROUND_FLOOR
import importlib.util
import json
from pathlib import Path


def main() -> None:
    """Enclose area 4 E(16/25); run `python3 reference_bounds.py` in any directory."""
    source = Path(__file__).parent.parent / "surface-property-policy/ellipse_area_bounds.py"
    specification = importlib.util.spec_from_file_location("elliptic_bounds", source)
    proof = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(proof)
    pi_low, pi_high = proof.pi_bounds()
    series_low, series_high = proof.elliptic_series_bounds(256, Fraction(16, 25))
    lower, upper = 2 * pi_low * (1 - series_high), 2 * pi_high * (1 - series_low)
    assert upper - lower < Fraction(1, 10**50)
    result = {"area_lower": proof.decimal_bound(lower, ROUND_FLOOR),
              "area_upper": proof.decimal_bound(upper, ROUND_CEILING),
              "formula": "4 E(16/25)", "exact_rational_arithmetic": True,
              "terms": 256, "tail_bound": "next term / (1-16/25)"}
    Path(__file__).with_name("reference-bounds.json").write_text(json.dumps(result, indent=2) + "\n")


if __name__ == "__main__":
    main()
