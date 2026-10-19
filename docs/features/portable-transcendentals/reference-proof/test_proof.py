"""Positive and fail-closed controls for the independent reference proof."""

import json
import unittest
from fractions import Fraction

import gmpy2
from exact_cells import exact_word, parse_cases, require_inside, rounding_cell, rational_text
from generate import directed, generate, prove_case
from verify import HERE, verify, verify_record


class ReferenceProofControls(unittest.TestCase):
    def test_word_decoding(self) -> None:
        self.assertEqual(exact_word(0x3ff0000000000000), 1)
        self.assertEqual(exact_word(0xbff0000000000000), -1)
        self.assertEqual(exact_word(1), Fraction(1, 2 ** 1074))
        self.assertEqual(exact_word(0x8000000000000000), 0)
        with self.assertRaises(ValueError):
            exact_word(0x7ff0000000000000)

    def test_multiline_parser(self) -> None:
        source = '("sin",0x3fe0_0000_0000_0000,0x3fde_aee8_744b_05f0),'
        self.assertEqual(parse_cases(source)[0][0], 'sin')
        self.assertEqual(len(parse_cases('(\n"pow",0x1,0x2,0x3,\n)')[0][1]), 2)

    def test_positive_negative_and_asymmetric_cells(self) -> None:
        left, right = rounding_cell(0x3ff0000000000000)
        self.assertEqual(1 - left, Fraction(1, 2 ** 54))
        self.assertEqual(right - 1, Fraction(1, 2 ** 53))
        self.assertEqual(rounding_cell(0xbff0000000000000), (-right, -left))
        require_inside(Fraction(1), Fraction(1), 0x3ff0000000000000)
        self.assertEqual(rational_text(Fraction(1, 2)), '1/2')

    def test_boundary_and_wrong_word_rejected(self) -> None:
        left, right = rounding_cell(0x3ff0000000000000)
        with self.assertRaises(ValueError):
            require_inside(left, right, 0x3ff0000000000000)
        with self.assertRaises(ValueError):
            require_inside(Fraction(1), Fraction(1), 0x3ff0000000000001)
        for word in (0, 0x8000000000000000, 0x7fefffffffffffff):
            with self.assertRaises(ValueError):
                rounding_cell(word)

    def test_directed_exact_value_and_scoped_special_rejection(self) -> None:
        for mode in (gmpy2.RoundDown, gmpy2.RoundUp):
            self.assertEqual(directed('exp', (0,), 256, mode), 1)
        with self.assertRaises(ValueError):
            prove_case(0, ('sin', (0,), 0))

    def test_all_records_recompute_identically(self) -> None:
        self.assertEqual(generate(), json.loads((HERE / 'enclosures.json').read_text()))
        verify()

    def test_tampered_endpoint_or_identity_rejected(self) -> None:
        case = ('sin', (0x3fe0000000000000,), 0x3fdeaee8744b05f0)
        record = prove_case(0, case)
        verify_record(record, case, 0)
        record['lower'] = '0/1'
        with self.assertRaises(ValueError):
            verify_record(record, case, 0)
        with self.assertRaises(AssertionError):
            verify_record(record, case, 1)


if __name__ == '__main__':
    unittest.main()
