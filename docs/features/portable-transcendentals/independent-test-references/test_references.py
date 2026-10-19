"""Independent reference generator and exact-input admission controls."""

import json
import re
import unittest
from fractions import Fraction

from generate_references import (HERE, ROOT, certify, norm_reference, planar_grid,
                                 primitive, rust_rows, spatial_jet, spatial_references, word)
from exact_cells import require_inside


class IndependentReferenceControls(unittest.TestCase):
    def test_exact_inputs_match_the_rust_source_words(self) -> None:
        self.assertEqual(word(1e200), 0x6974e718d7d7625a)
        self.assertEqual(word(0.5), 0x3fe0000000000000)
        self.assertEqual(word(1.0), 0x3ff0000000000000)
        rows, _ = planar_grid()
        self.assertEqual([row[0] for row in rows], [word(index / 8.0) for index in range(-16, 17)])

    def test_all_archived_primitive_cells_and_exact_zeros(self) -> None:
        records = json.loads((HERE / 'certificates.json').read_text())['proofs']
        self.assertEqual(len(records), 154)
        for record in records:
            lower, upper = Fraction(record['lower']), Fraction(record['upper'])
            if record['kind'] == 'exact-zero':
                self.assertEqual((lower, upper, record['word']), (0, 0, '0000000000000000'))
                continue
            require_inside(lower, upper, int(record['word'], 16))

    def test_scalar_closed_forms_have_independent_words(self) -> None:
        integral = primitive('expm1', (word(1.0),))
        self.assertEqual(integral['word'], '3ffb7e151628aed3')
        norm = norm_reference()
        self.assertEqual(norm['word'], '697d8f9811335b57')
        self.assertEqual(norm['arguments'], ['6974e718d7d7625a'])
        self.assertEqual(certify('zero', Fraction(0), Fraction(0))['kind'], 'exact-zero')

    def test_generated_planar_words_match_the_ordinary_test_table(self) -> None:
        rows, _ = planar_grid()
        source = (ROOT / 'crates/martensite-tkg2d/tests/support/analytic_reference_words.rs').read_text()
        words = [int(value, 16) for value in re.findall(r'0x[0-9a-f]+', source)]
        self.assertEqual(words, [value for row in rows for value in row])
        self.assertEqual(len(re.findall('0x', rust_rows('TEST', rows))), 165)

    def test_spatial_words_and_derivative_cycles_match(self) -> None:
        _, proofs = spatial_references()
        self.assertEqual(len(proofs), 20)
        source = (ROOT / 'crates/martensite-tkmath/tests/support/elementary_reference_words.rs').read_text()
        expected = []
        for family, argument in [('circle', 0x3fe921fb54442d18), ('ellipse', 0x3fe921fb54442d18),
                                 ('hyperbola', 0x3fe0000000000000)]:
            jet, _ = spatial_jet(family, argument)
            expected.extend(value for row in jet for value in row)
        self.assertEqual([int(value, 16) for value in re.findall(r'0x[0-9a-f]+', source)], expected)

    def test_full_certificate_regeneration_is_identical(self) -> None:
        _, planar = planar_grid()
        _, spatial = spatial_references()
        expected = planar + spatial + [primitive('expm1', (word(1.0),)), norm_reference()]
        self.assertEqual(expected, json.loads((HERE / 'certificates.json').read_text())['proofs'])

    def test_inconclusive_enclosure_is_rejected(self) -> None:
        with self.assertRaises(ValueError):
            certify('crosses-cell', Fraction(1), Fraction(2))
        with self.assertRaises(ValueError):
            certify('reversed', Fraction(1), Fraction(0))


if __name__ == '__main__':
    unittest.main()
