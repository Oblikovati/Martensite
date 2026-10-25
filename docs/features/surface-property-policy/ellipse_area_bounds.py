"""Exact rational bounds for the test ellipse, independent of kernel integration."""

from decimal import Decimal, ROUND_CEILING, ROUND_FLOOR, localcontext
from fractions import Fraction
import json
from pathlib import Path


def atan_bounds(denominator: int, terms: int) -> tuple[Fraction, Fraction]:
    """Bound atan(1/d) by its alternating Taylor series."""
    partial = sum(
        (
            Fraction((-1) ** k, (2 * k + 1) * denominator ** (2 * k + 1))
            for k in range(terms)
        ),
        Fraction(),
    )
    omitted = Fraction(1, (2 * terms + 1) * denominator ** (2 * terms + 1))
    return (
        (partial, partial + omitted) if terms % 2 == 0 else (partial - omitted, partial)
    )


def pi_bounds() -> tuple[Fraction, Fraction]:
    """Use Machin's identity with rigorous alternating-series remainders."""
    a_low, a_high = atan_bounds(5, 80)
    b_low, b_high = atan_bounds(239, 24)
    return 16 * a_low - 4 * b_high, 16 * a_high - 4 * b_low


def elliptic_series_bounds(terms: int) -> tuple[Fraction, Fraction]:
    """E(m)/(pi/2)=1-sum(c_n²*m^n/(2n-1)), m=3/4; next ratios <m."""
    parameter, coefficient, power = Fraction(3, 4), Fraction(1), Fraction(1)
    total = Fraction()
    for n in range(1, terms + 1):
        coefficient *= Fraction(2 * n - 1, 2 * n)
        power *= parameter
        total += coefficient * coefficient * power / (2 * n - 1)
    next_coefficient = coefficient * Fraction(2 * terms + 1, 2 * terms + 2)
    next_term = next_coefficient**2 * power * parameter / (2 * terms + 1)
    return total, total + next_term / (1 - parameter)


def decimal_bound(value: Fraction, rounding: str) -> str:
    """Render an outward-rounded decimal endpoint without changing the rational proof."""
    with localcontext() as context:
        context.prec, context.rounding = 70, rounding
        return str(Decimal(value.numerator) / Decimal(value.denominator))


def main() -> None:
    """Enclose the prism area 4*pi + 24*E(3/4) for semi-axes 2/1 and height 3."""
    pi_low, pi_high = pi_bounds()
    series_low, series_high = elliptic_series_bounds(256)
    lower = pi_low * (16 - 12 * series_high)
    upper = pi_high * (16 - 12 * series_low)
    assert upper - lower < Fraction(1, 10**34)
    result = {
        "area_lower": decimal_bound(lower, ROUND_FLOOR),
        "area_upper": decimal_bound(upper, ROUND_CEILING),
        "interval_width_upper": decimal_bound(upper - lower, ROUND_CEILING),
        "exact_rational_arithmetic": True,
        "terms": 256,
        "tail_bound": "next positive term / (1-m), with every following ratio < m=3/4",
    }
    Path(__file__).with_name("ellipse-area-bounds.json").write_text(
        json.dumps(result, indent=2) + "\n"
    )
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
