"""Attribute every changed fixture row to its replayed tests and differing libm calls (#201).

Inputs (all produced by replay.sh):
  * the two `cargo test` logs (platform replay of the original tests and fixtures,
    portable replay of the candidate tests and fixtures),
  * the two call-log directories written by the `libm-attribution` feature,
  * `binaries.json`, mapping each test executable to its package,
  * `calls.tsv.xz` from verify_mpfr.py.

A row is `attributed` when every test that consumes it passed in both replays (old
expected bytes under platform libm, new expected bytes under portable functions),
those tests made at least one differing call, and every finite differing call in
them was verified against MPFR as portable correctly rounded and platform not.

    python3 -I attribute_rows.py --evidence REPLAY_OUT_DIR

It updates ../changed-rows-*.json and writes test-call-digests.json next to this script.
"""

import argparse
import gzip
import hashlib
import json
import lzma
import re
from collections import defaultdict
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[3]
INVENTORY = HERE.parent
SHARD_COUNT = 7  # changed-rows-000.json to changed-rows-006.json


def shard_path(index: int) -> Path:
    """Fixed inventory shard location; never derived from input data."""
    return INVENTORY / f'changed-rows-{index:03d}.json'
EVIDENCE = 'docs/features/portable-transcendentals/attribution/README.md'

RESULT = re.compile(r'^test (\S+) \.\.\. (ok|FAILED|ignored)$')

# Consumers whose test name is not derived from the row's request family.
FIXED_CONSUMERS = {
    'crates/martensite-tkg3d/tests/numeric_replay/fixtures/expected.txt.gz':
        ('martensite-tkg3d', 'numeric_replay::version_one_numeric_corpus_replays_all_requests_in_process'),
    'crates/martensite-tkgeombase/tests/fixtures/ordinary_curve/baseline.txt':
        ('martensite-tkgeombase', 'ordinary_curve_cohort::odd_rule_nonlinear_cut_cap_retains_middle_correction_behavior'),
    'crates/martensite-tkgeombase/tests/fixtures/bivariate_legacy/answers.txt':
        ('martensite-tkgeombase', 'nineteen_families_preserve_all_requests_outputs_and_order'),
}
# Families whose replay test is not named after the request token.
FAMILY_TEST_NAMES = {'acc_ls': 'continuous_least_squares'}
EXPRESSION_FAMILIES = {
    'ex_text': 'semantic_corpus::interpreter_sequences_and_seeded_invalid_tokens_replay',
    'ex_fold': 'semantic_corpus::unary_functions_special_values_and_random_values_replay',
}


def test_results(log: Path, binaries: dict) -> dict:
    """(package, test name) -> 'ok' | 'FAILED' | 'ignored' for one cargo test log."""
    results, package = {}, None
    for line in log.read_text(errors='replace').splitlines():
        if line.lstrip().startswith('Running ') and '/deps/' in line:
            package = binaries[line.rstrip().rstrip(')').rsplit('/deps/', 1)[1]]
        elif (match := RESULT.match(line)) and package:
            results[(package, match.group(1))] = match.group(2)
    return results


def call_index(directory: Path, binaries: dict) -> dict:
    """(package, test) -> set of (function, inputs) differing calls."""
    calls = defaultdict(set)
    for log in sorted(directory.glob('*.jsonl')):
        for line in log.read_text().splitlines():
            record = json.loads(line)
            package = binaries.get(record['binary'], 'doc-tests')
            calls[(package, record['test'])].add((record['function'], tuple(record['inputs'])))
    return calls


def request_path(path: str) -> str:
    name = Path(path).name
    for answer, request in (('expected-', 'inputs-'), ('native.txt', 'inputs.txt'),
                            ('answers.txt', 'requests.txt'), ('expected.txt.gz', 'requests.txt.gz'),
                            ('expected.txt', 'inputs.txt')):
        if name.startswith(answer):
            return str(Path(path).with_name(name.replace(answer, request, 1)))
    raise ValueError(f'no request file paired with {path}; expected an answer fixture name')


def read_lines(path: str) -> list[str]:
    source = (REPO / path).resolve()
    if not source.is_relative_to(REPO / 'crates'):
        raise ValueError(f'fixture path {path!r} leaves the crates directory; expected crates/...')
    raw = source.read_bytes()
    text = gzip.decompress(raw) if path.endswith('.gz') else raw
    return text.decode().split('\n')


def consumers(row: dict, requests: list[str], tests: set) -> list[tuple[str, str]]:
    path = row['path']
    if path in FIXED_CONSUMERS:
        return [FIXED_CONSUMERS[path]]
    package = path.split('/')[1]
    family = requests[row['line_one_based'] - 1].split()[0]
    if family in EXPRESSION_FAMILIES:
        return [(package, EXPRESSION_FAMILIES[family])]
    family = FAMILY_TEST_NAMES.get(family, family)
    corpus = Path(path).parent.name
    found = [(p, t) for p, t in tests if p == package and
             (t.split('::')[-1] in (family, f'{family}_complete_family')) and
             (corpus in t.split('::') or 'numeric_replay' in path)]
    return sorted(found)


def mpfr_verdicts(path: Path) -> dict:
    """(function, inputs) -> {'value_class', 'verdict'} from verify_mpfr.py's table."""
    verdicts = {}
    with lzma.open(path, 'rt') as stream:
        next(stream)
        for line in stream:
            function, inputs, _, _, _, value_class, verdict, _ = line.rstrip('\n').split('\t')
            verdicts[(function, tuple(inputs.split(',')))] = {'value_class': value_class, 'verdict': verdict}
    return verdicts


NAN_TEXT = '6e616e'  # hex of "nan" in the expression corpus' text tokens


def row_cause(row: dict) -> str:
    """`domain_error_nan_sign` when every changed token is NaN text whose sign changed
    ("-nan" to "nan"); otherwise `rounding`."""
    tokens = row['changed_tokens']
    if tokens and all(NAN_TEXT in (t['old'] or '') and NAN_TEXT in (t['new'] or '') for t in tokens):
        return 'domain_error_nan_sign'
    return 'rounding'


def attribute(row: dict, tests: list, runs: dict, calls: set, verdicts: dict) -> dict:
    statuses = {f'{p}::{t}': {name: run.get((p, t), 'not_run') for name, run in runs.items()} for p, t in tests}
    finite = [verdicts[c] for c in calls if verdicts[c].get('value_class') == 'finite']
    special = [verdicts[c] for c in calls if verdicts[c].get('value_class') != 'finite']
    reasons = []
    if not tests:
        reasons.append('no consuming test found')
    if any(s != {'platform_original': 'ok', 'portable_candidate': 'ok'} for s in statuses.values()):
        reasons.append('a consuming test did not pass in both replays')
    if not calls:
        reasons.append('no differing libm call in the consuming tests')
    if any(v['verdict'] != 'portable_correctly_rounded_platform_not' for v in finite):
        reasons.append('a finite differing call is not portable-correct/platform-incorrect')
    cause = row_cause(row)
    if cause == 'rounding' and not finite:
        reasons.append('no finite differing call explains a value change')
    if cause == 'domain_error_nan_sign' and not any(v['verdict'] == 'nan_payload_or_sign_only' for v in special):
        reasons.append('no NaN-sign differing call explains a NaN text change')
    functions = defaultdict(int)
    for function, _ in calls:
        functions[function] += 1
    status = 'unexplained' if reasons else 'attributed'
    if status == 'attributed' and cause == 'domain_error_nan_sign':
        # A NaN sign is not a correctly rounded result; #203 owns these rows.
        status = 'not_admitted_nan_sign_issue_203'
    return {
        'independent_validation': status,
        'attribution': {
            'evidence': EVIDENCE,
            'cause': cause,
            'tests': statuses,
            'differing_calls': len(calls),
            'finite_rounding_calls_verified': len(finite),
            'special_value_calls': len(special),
            'functions': dict(sorted(functions.items())),
            **({'unexplained_reasons': reasons} if reasons else {}),
        },
    }


def call_digest(calls: set) -> dict:
    """Count, per-function counts and SHA-256 of the sorted `function(inputs)` keys,
    so the per-test call sets can be recomputed from the replay logs and compared."""
    keys = sorted(f'{f}({",".join(i)})' for f, i in calls)
    functions = defaultdict(int)
    for function, _ in calls:
        functions[function] += 1
    return {'differing_calls': len(keys), 'functions': dict(sorted(functions.items())),
            'sha256': hashlib.sha256('\n'.join(keys).encode()).hexdigest()}


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument('--evidence', type=Path, required=True)
    options = parser.parse_args()
    evidence = options.evidence
    binaries = json.loads((evidence / 'binaries.json').read_text())
    runs = {name: test_results(evidence / f'{name}.cargo-test.log', binaries)
            for name in ('platform_original', 'portable_candidate')}
    calls = defaultdict(set)
    for name in runs:
        for key, value in call_index(evidence / f'calls-{name}', binaries).items():
            calls[key] |= value
    verdicts = mpfr_verdicts(evidence / 'calls.tsv.xz')
    all_tests = set(runs['portable_candidate']) | set(runs['platform_original'])
    summary = defaultdict(int)
    request_cache: dict[str, list[str]] = {}
    for index in range(SHARD_COUNT):
        rows = json.loads(shard_path(index).read_text())
        for row in rows:
            path = row['path']
            if path not in FIXED_CONSUMERS and path not in request_cache:
                request_cache[path] = read_lines(request_path(path))
            requests = request_cache.get(path, [])
            tests = consumers(row, requests, all_tests)
            row_calls = set().union(*(calls[t] for t in tests)) if tests else set()
            row.update(attribute(row, tests, runs, row_calls, verdicts))
            summary[row['independent_validation']] += 1
        lines = ',\n'.join(json.dumps(row, separators=(',', ':')) for row in rows)
        with open(shard_path(index), 'w', encoding='utf-8') as stream:
            stream.write(f'[\n{lines}\n]\n')
    per_test = {f'{p}::{t}': call_digest(v) for (p, t), v in sorted(calls.items())}
    (HERE / 'test-call-digests.json').write_text(json.dumps(per_test, indent=1, sort_keys=True) + '\n')
    print(json.dumps(summary))


if __name__ == '__main__':
    main()
