"""Independent exact-input MPFR certificates for composed test references (#201)."""

import json
import struct
import sys
from fractions import Fraction
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
sys.path.insert(0, str(HERE.parent / 'reference-proof'))
from exact_cells import exact_word, rational_text, require_inside
from generate import directed
import gmpy2


def word(value: float) -> int:
    """Encode exact binary64 input; example: word(0.5) is 0x3fe0000000000000."""
    encoded = struct.pack('>d', value)
    return int.from_bytes(encoded, 'big')


def certify(label: str, lower: Fraction, upper: Fraction) -> dict[str, object]:
    """Discover a word from a rational, then prove it by strict enclosure."""
    expected = word(float(lower))
    if lower == upper == 0:
        return {'label': label, 'word': '0000000000000000', 'kind': 'exact-zero',
                'lower': '0/1', 'upper': '0/1'}
    require_inside(lower, upper, expected)
    return {'label': label, 'word': f'{expected:016x}', 'kind': 'strict-cell',
            'lower': rational_text(lower), 'upper': rational_text(upper)}


def primitive(name: str, arguments: tuple[int, ...], scale: int = 1) -> dict[str, object]:
    """Enclose the real scaled function, not a candidate-rounded intermediate."""
    lower = directed(name, arguments, 256, gmpy2.RoundDown) * scale
    upper = directed(name, arguments, 256, gmpy2.RoundUp) * scale
    record = certify(f'{scale}*{name}{arguments}', min(lower, upper), max(lower, upper))
    return dict(record, function=name, arguments=[f'{value:016x}' for value in arguments], scale=scale)


def planar_grid() -> tuple[list[list[int]], list[dict[str, object]]]:
    """All 33 dyadic parameters are exact; reference arithmetic has no host libm."""
    rows: list[list[int]] = []
    proofs: list[dict[str, object]] = []
    for index in range(-16, 17):
        argument = word(index / 8.0)
        records = [primitive(name, (argument,)) for name in ('sin', 'cos', 'sinh', 'cosh')]
        rows.append([argument] + [int(str(record['word']), 16) for record in records])
        proofs.extend(records)
    return rows, proofs


def spatial_jet(family: str, argument: int) -> tuple[list[list[int]], list[dict[str, object]]]:
    """Closed forms use the original identity frame and exact radii 2 or 3."""
    a, b = (2, 2) if family == 'circle' else (3, 2)
    first, second = ('cosh', 'sinh') if family == 'hyperbola' else ('cos', 'sin')
    specs = [(a, first), (b, second), (a, second), (b, first)]
    if family != 'hyperbola':
        specs += [(-a, second), (-a, first), (-b, second), (-b, first)]
    proofs = [primitive(name, (argument,), scale) for scale, name in specs]
    values = [int(str(record['word']), 16) for record in proofs]
    if family == 'hyperbola':
        return [[values[0], values[1], 0], [values[2], values[3], 0]] * 2, proofs
    return [[values[0], values[1], 0], [values[4], values[3], 0],
            [values[5], values[6], 0], [values[2], values[7], 0]], proofs


def rust_rows(name: str, rows: list[list[int]]) -> str:
    """Keep generated words compact and separate from test control flow."""
    result = f'pub(super) const {name}: [[u64; {len(rows[0])}]; {len(rows)}] = [\n'
    result += ''.join('    [' + ', '.join(f'0x{value:016x}' for value in row) + '],\n' for row in rows)
    return result + '];\n'


def spatial_references() -> tuple[str, list[dict[str, object]]]:
    """Keep each fixed radius and actual dyadic parameter explicit."""
    text = '//! Independent MPFR references for exact test inputs; see #201 evidence.\n'
    proofs: list[dict[str, object]] = []
    for family, argument in [('circle', 0x3fe921fb54442d18), ('ellipse', 0x3fe921fb54442d18),
                             ('hyperbola', 0x3fe0000000000000)]:
        jet, records = spatial_jet(family, argument)
        text += rust_rows(f'{family.upper()}_REFERENCE_JET', jet)
        proofs.extend(records)
    return text, proofs


def norm_reference() -> dict[str, object]:
    """Use abs(x)*sqrt(2) for the exact stored x, without a hypot oracle call."""
    argument = word(1e200)
    bounds: list[Fraction] = []
    for mode in (gmpy2.RoundDown, gmpy2.RoundUp):
        with gmpy2.context(precision=256, round=mode):
            numerator, denominator = gmpy2.sqrt(gmpy2.mpfr(2)).as_integer_ratio()
            bounds.append(exact_word(argument) * Fraction(int(numerator), int(denominator)))
    record = certify('abs(x)*sqrt(2)', *bounds)
    return dict(record, function='closed_form_norm', arguments=[f'{argument:016x}'], scale=1)


def generate_references() -> None:
    """Write only owned test references/proofs; run this script from any cwd."""
    grid, proofs = planar_grid()
    header = '//! Independent MPFR references for exact test inputs; see #201 evidence.\n'
    (ROOT / 'crates/martensite-tkg2d/tests/support/analytic_reference_words.rs').write_text(
        header + rust_rows('ANALYTIC_REFERENCE_WORDS', grid))
    spatial, records = spatial_references()
    (ROOT / 'crates/martensite-tkmath/tests/support/elementary_reference_words.rs').write_text(spatial)
    proofs += records + [primitive('expm1', (word(1.0),)), norm_reference()]
    (HERE / 'certificates.json').write_text(json.dumps({'precision': 256, 'proofs': proofs}) + '\n')


if __name__ == '__main__':
    generate_references()
