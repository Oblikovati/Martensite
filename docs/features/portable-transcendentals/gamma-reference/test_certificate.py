"""Independent operand and enclosure controls for inverse gamma."""
import copy
import hashlib
from fractions import Fraction
import json
from pathlib import Path
import unittest

from gamma_certificate import certificate, inverse_exponent, rounded_power_word
from verify_binary32 import verify_binary32_record

HERE = Path(__file__).resolve().parent


class GammaCertificateTests(unittest.TestCase):
    def test_reused_sources_match_recorded_identities(self) -> None:
        bindings = json.loads((HERE / 'source-bindings.json').read_text())
        root = HERE.parents[3]
        for relative, expected in bindings['sha256'].items():
            self.assertEqual(hashlib.sha256((root / relative).read_bytes()).hexdigest(), expected)

    def test_exact_division_selects_the_lower_neighbor(self) -> None:
        division = inverse_exponent()
        self.assertEqual(division['word'], '3ee8ba2e')
        self.assertEqual(Fraction(division['exact_reciprocal']), Fraction(4194304, 9227469))

    def test_regenerated_certificate_matches_all_retained_fields(self) -> None:
        retained = json.loads((HERE / 'certificate.json').read_text())
        self.assertEqual(certificate(), retained)
        self.assertEqual(rounded_power_word(0x3EE8BA2E), 0x3F3AD03F)

    def test_independent_cell_rejects_the_adjacent_expected_word(self) -> None:
        record = certificate()['power']
        verify_binary32_record(record, (0x3F000000, 0x3EE8BA2E, 0x3F3AD03F), 0)
        altered = copy.deepcopy(record)
        altered['expected'] = '3f3ad03e'
        with self.assertRaises((AssertionError, ValueError)):
            verify_binary32_record(altered, (0x3F000000, 0x3EE8BA2E, 0x3F3AD03E), 0)


if __name__ == '__main__':
    unittest.main()
