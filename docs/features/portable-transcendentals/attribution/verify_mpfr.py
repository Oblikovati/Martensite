"""Check every distinct differing libm call against MPFR (#201).

Reads the attribution call logs (JSON lines written by the martensite-cmath
`libm-attribution` feature), deduplicates them by function and input words, and
for each one computes the exactly rounded binary64 (binary32 for `powf`) result
with MPFR in two ways:

* directly at the destination precision (MPFR's own correct rounding), and
* at 256 bits, then rounded once to the destination format.

Both must agree. The verdict records whether the portable word and the platform
(glibc) word equal that reference. Run with the pinned gmpy2 from
../reference-proof/requirements.txt:

    python3 -I verify_mpfr.py LOG_DIR [LOG_DIR ...]

It writes `calls.tsv.xz` and `mpfr-summary.json` next to this script.
`calls.tsv.xz` holds one line per distinct call: function, comma-separated input
words, platform result words, portable result words, MPFR result words, value
class, verdict, and whether calling the running C library (ctypes) again gives the
platform words (words are hexadecimal bit patterns).
"""

import argparse
import ctypes
import ctypes.util
import lzma
import json
import math
import struct
import sys
from pathlib import Path

import gmpy2
from gmpy2 import mpfr

BINARY64 = {'precision': 53, 'emin': -1073, 'emax': 1024, 'subnormalize': True}
BINARY32 = {'precision': 24, 'emin': -148, 'emax': 128, 'subnormalize': True}
WIDE_BITS = 256
HERE = Path(__file__).resolve().parent


def f64(word: str) -> float:
    return struct.unpack('<d', struct.pack('<Q', int(word, 16)))[0]


def f32(word: str) -> float:
    return struct.unpack('<f', struct.pack('<I', int(word, 16)))[0]


def word64(value: float) -> str:
    return f'{struct.unpack("<Q", struct.pack("<d", value))[0]:016x}'


def word32(value: float) -> str:
    return f'{struct.unpack("<I", struct.pack("<f", value))[0]:016x}'


UNARY = {
    'sin': gmpy2.sin, 'cos': gmpy2.cos, 'tan': gmpy2.tan,
    'asin': gmpy2.asin, 'acos': gmpy2.acos, 'atan': gmpy2.atan,
    'sinh': gmpy2.sinh, 'cosh': gmpy2.cosh, 'tanh': gmpy2.tanh,
    'asinh': gmpy2.asinh, 'acosh': gmpy2.acosh, 'atanh': gmpy2.atanh,
    'exp': gmpy2.exp, 'exp2': gmpy2.exp2, 'expm1': gmpy2.expm1,
    'log': gmpy2.log, 'log1p': gmpy2.log1p, 'log2': gmpy2.log2, 'log10': gmpy2.log10,
    'cbrt': gmpy2.cbrt,
}
BINARY = {
    'atan2': gmpy2.atan2, 'hypot': gmpy2.hypot, 'pow': lambda x, y: x ** y,
    'powf': lambda x, y: x ** y,
}


def evaluate(function: str, args: list[float]) -> list:
    """Exact-function values at the current context precision (MPFR rounding)."""
    values = [mpfr(arg) for arg in args]
    if function in UNARY:
        return [UNARY[function](values[0])]
    if function == 'sincos':
        return [gmpy2.sin(values[0]), gmpy2.cos(values[0])]
    if function in BINARY:
        return [BINARY[function](values[0], values[1])]
    raise ValueError(f'no MPFR reference for {function!r}; expected one of the adapter functions')


def reference(function: str, args: list[float], fmt: dict) -> list:
    """Correctly rounded result directly at the destination format."""
    with gmpy2.context(gmpy2.ieee(64), **fmt):
        return [ctx_round(value) for value in evaluate(function, args)]


def ctx_round(value):
    return gmpy2.check_range(+value)


def wide_then_round(function: str, args: list[float], fmt: dict) -> list:
    """Evaluate at WIDE_BITS, then round once to the destination format."""
    with gmpy2.context(gmpy2.ieee(64), precision=WIDE_BITS, emin=-100000, emax=100000):
        wide = evaluate(function, args)
    with gmpy2.context(gmpy2.ieee(64), **fmt):
        return [ctx_round(mpfr(value)) for value in wide]


def words_of(values: list, single: bool) -> list[str]:
    to_word = word32 if single else word64
    return [to_word(float(value)) for value in values]


def is_nan_word(word: str, single: bool) -> bool:
    return math.isnan(f32(word) if single else f64(word))


def is_finite_word(word: str, single: bool) -> bool:
    return math.isfinite(f32(word) if single else f64(word))


def value_class(call: dict, mpfr_words: list[str], single: bool) -> str:
    """`finite` when every input and both reference and platform results are finite;
    such a difference is a rounding difference. Otherwise it is a special-value case."""
    words = call['inputs'] + call['platform'] + mpfr_words
    return 'finite' if all(is_finite_word(w, single) for w in words) else 'special_value'


def verdict(call: dict, mpfr_words: list[str], single: bool) -> str:
    portable, platform = call['portable'], call['platform']
    if all(is_nan_word(w, single) for w in portable + platform + mpfr_words):
        return 'nan_payload_or_sign_only'
    portable_ok = portable == mpfr_words
    platform_ok = platform == mpfr_words
    return {
        (True, False): 'portable_correctly_rounded_platform_not',
        (True, True): 'both_correct',
        (False, True): 'platform_correctly_rounded_portable_not',
        (False, False): 'neither_correctly_rounded',
    }[(portable_ok, platform_ok)]


LIBM = ctypes.CDLL(ctypes.util.find_library('m'))
# Pre-#201 platform entry points that are the C library itself. powi was Rust's
# repeated squaring and cbrt the in-tree glibc 2.39 port; both are not checked here.
GLIBC_NAMES = {'log': 'log', 'pow': 'pow', 'powf': 'powf', **{n: n for n in UNARY if n != 'cbrt'},
               'atan2': 'atan2', 'hypot': 'hypot'}


def glibc_words(function: str, args: list[float]) -> list[str] | None:
    """Recompute the platform result by calling the running C library directly."""
    if function == 'sincos':
        return [word64(glibc_call('sin', args, False)), word64(glibc_call('cos', args, False))]
    if function not in GLIBC_NAMES:
        return None
    single = function == 'powf'
    value = glibc_call(GLIBC_NAMES[function], args, single)
    return [word32(value) if single else word64(value)]


def glibc_call(name: str, args: list[float], single: bool) -> float:
    entry = getattr(LIBM, name)
    kind = ctypes.c_float if single else ctypes.c_double
    entry.restype, entry.argtypes = kind, [kind] * len(args)
    return entry(*args)


def glibc_agreement(libc: list[str] | None, platform: list[str], single: bool) -> str:
    if libc is None:
        return 'n/a'
    if libc == platform:
        return 'equal'
    if all(is_nan_word(w, single) for w in libc + platform):
        return 'nan_differs'
    return 'differs'


def check(call: dict) -> dict:
    function = call['function']
    single = function == 'powf'
    if function == 'powi':
        return dict(call, mpfr=None, glibc='n/a', verdict='not_a_libm_function')
    args = [f32(w) if single else f64(w) for w in call['inputs']]
    call = dict(call, glibc=glibc_agreement(glibc_words(function, args), call['platform'], single))
    fmt = BINARY32 if single else BINARY64
    direct = words_of(reference(function, args, fmt), single)
    wide = words_of(wide_then_round(function, args, fmt), single)
    if direct != wide:
        return dict(call, mpfr=direct, mpfr_wide=wide, verdict='reference_disagreement')
    return dict(call, mpfr=direct, value_class=value_class(call, direct, single),
                verdict=verdict(call, direct, single))


def distinct_calls(directories: list[Path]) -> dict:
    calls: dict = {}
    for directory in directories:
        for log in sorted(directory.glob('*.jsonl')):
            for line in log.read_text().splitlines():
                record = json.loads(line)
                key = (record['function'], tuple(record['inputs']))
                previous = calls.setdefault(key, {k: record[k] for k in ('function', 'inputs', 'platform', 'portable')})
                if (previous['platform'], previous['portable']) != (record['platform'], record['portable']):
                    raise ValueError(f'nondeterministic results for {key}: {previous} vs {record}')
    return calls


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument('logs', nargs='+', type=Path)
    options = parser.parse_args()
    calls = distinct_calls(options.logs)
    results = [check(calls[key]) for key in sorted(calls)]
    summary: dict = {}
    for result in results:
        bucket = summary.setdefault(result['function'], {})
        label = f"{result.get('value_class', 'n/a')}:{result['verdict']}:glibc_{result['glibc']}"
        bucket[label] = bucket.get(label, 0) + 1
    document = {
        'schema': 1,
        'gmpy2': gmpy2.version(), 'mpfr': gmpy2.mpfr_version(), 'gmp': gmpy2.mp_version(),
        'python': sys.version.split()[0],
        'wide_precision_bits': WIDE_BITS,
        'distinct_calls': len(results),
        'summary': summary,
    }
    with lzma.open(HERE / 'calls.tsv.xz', 'wt', preset=9) as stream:
        stream.write('function\tinputs\tplatform\tportable\tmpfr\tvalue_class\tverdict\tglibc\n')
        for r in results:
            words = [','.join(r[k] or []) for k in ('inputs', 'platform', 'portable', 'mpfr')]
            fields = [r['function'], *words, r.get('value_class', 'n/a'), r['verdict'], r['glibc']]
            stream.write('\t'.join(fields) + '\n')
    (HERE / 'mpfr-summary.json').write_text(json.dumps(document, indent=1, sort_keys=True) + '\n')
    print(json.dumps(document, indent=1, sort_keys=True))


if __name__ == '__main__':
    main()
