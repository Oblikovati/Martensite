"""Approximate Rust source metrics for a list of files (brace counting, not a parser)."""
import re
import sys
from collections import Counter

FN_RE = re.compile(r'^\s*(pub(\([^)]*\))?\s+)?(const\s+)?(async\s+)?(unsafe\s+)?fn\s+(\w+)')
CFG_TEST_RE = re.compile(r'#\[cfg\(test\)\]')


def strip_line(line: str, state: dict) -> str:
    """Remove comments and string/char literal contents so braces are counted only in code."""
    out = []
    i = 0
    n = len(line)
    while i < n:
        if state['block']:
            j = line.find('*/', i)
            if j < 0:
                return ''.join(out)
            state['block'] = False
            i = j + 2
            continue
        if state['raw'] is not None:
            j = line.find(state['raw'], i)
            if j < 0:
                return ''.join(out)
            i = j + len(state['raw'])
            state['raw'] = None
            out.append('""')
            continue
        if state['str']:
            c = line[i]
            if c == '\\':
                i += 2
                continue
            if c == '"':
                state['str'] = False
                out.append('"')
            i += 1
            continue
        c = line[i]
        if line.startswith('//', i):
            break
        if line.startswith('/*', i):
            state['block'] = True
            i += 2
            continue
        m = re.match(r'b?r(#*)"', line[i:])
        if m and (i == 0 or not (line[i - 1].isalnum() or line[i - 1] == '_')):
            state['raw'] = '"' + m.group(1)
            i += len(m.group(0))
            continue
        if c == '"':
            state['str'] = True
            out.append('"')
            i += 1
            continue
        if c == "'":
            m = re.match(r"'(\\.|[^\\'])'", line[i:]) or re.match(r"'\\u\{[0-9a-fA-F]+\}'", line[i:])
            if m:
                out.append("' '")
                i += len(m.group(0))
                continue
        out.append(c)
        i += 1
    return ''.join(out)


def is_test_path(path: str) -> bool:
    parts = path.split('/')
    return (
        '/tests/' in path
        or parts[-1] in ('tests.rs', 'test.rs')
        or parts[-1].endswith('_tests.rs')
        or parts[-1].startswith('tests_')
    )


def analyze(path: str, acc: dict) -> None:
    lines = open(path, encoding='utf-8').read().split('\n')
    test_file = is_test_path(path)
    state = {'block': False, 'str': False, 'raw': None}
    depth = 0
    pending_cfg_test = False
    test_until_depth = None  # inside #[cfg(test)] item while depth > this
    fn_stack = []  # (name, start_line, body_depth, is_test)
    pending_fn = None
    acc['files'] += 1
    acc['lines_test' if test_file else 'lines_src'] += len(lines)
    if len(lines) > 500:
        acc['big_files'].append((len(lines), path))
    for ln, raw in enumerate(lines, 1):
        code = strip_line(raw, state)
        in_test = test_file or test_until_depth is not None
        if CFG_TEST_RE.search(code):
            pending_cfg_test = True
            if not test_file and re.search(r'#\[cfg\(test\)\]\s*$', code.strip()) is not None:
                pass
        if re.search(r'^\s*(pub(\([^)]*\))?\s+)?mod\s+\w+\s*\{', code) and pending_cfg_test and not test_file:
            acc['cfg_test_inline_mods'] += 1
        if re.search(r'^\s*(pub(\([^)]*\))?\s+)?mod\s+\w+\s*;', code) and pending_cfg_test and not test_file:
            acc['cfg_test_file_mods'] += 1
        if 'allow(dead_code)' in code:
            acc['dead_code'] += 1
        if re.search(r'\bunsafe\b', code):
            acc['unsafe'] += 1
        if not in_test:
            acc['unwrap'] += len(re.findall(r'\.unwrap\(\)', code))
            acc['expect'] += len(re.findall(r'\.expect\(', code))
            acc['panic'] += len(re.findall(r'\b(panic|unreachable|todo|unimplemented)!', code))
            if re.match(r'^\s*pub\(crate\)\s', code):
                acc['pub_crate'] += 1
            elif re.match(r'^\s*pub(\(super\)|\(in [^)]*\))\s', code):
                acc['pub_super'] += 1
            elif re.match(r'^\s*pub\s', code):
                acc['pub'] += 1
        m = FN_RE.match(code)
        if m:
            pending_fn = (m.group(6), ln, in_test)
        for ch in code:
            if ch == '{':
                depth += 1
                if pending_cfg_test and test_until_depth is None and not test_file:
                    test_until_depth = depth - 1
                    pending_cfg_test = False
                if pending_fn is not None:
                    fn_stack.append([pending_fn[0], pending_fn[1], depth, pending_fn[2], 0])
                    pending_fn = None
                for f in fn_stack:
                    f[4] = max(f[4], depth - f[2])
            elif ch == '}':
                if fn_stack and fn_stack[-1][2] == depth:
                    name, start, _, ftest, nest = fn_stack.pop()
                    length = ln - start + 1
                    key = 'test' if ftest else 'src'
                    acc['fns_' + key] += 1
                    if length > 20:
                        acc['long_' + key].append((length, path, name))
                    acc['nest_' + key].append((nest, path, name))
                depth -= 1
                if test_until_depth is not None and depth == test_until_depth:
                    test_until_depth = None
            elif ch == ';':
                if pending_fn is not None and depth >= 0:
                    pending_fn = None  # trait method declaration without body
                if pending_cfg_test and test_until_depth is None:
                    pending_cfg_test = False


def main() -> None:
    acc = Counter()
    for k in ('big_files', 'long_src', 'long_test', 'nest_src', 'nest_test'):
        acc[k] = []
    for path in open(sys.argv[1]).read().split():
        analyze(path, acc)
    for k in ('files', 'lines_src', 'lines_test', 'fns_src', 'fns_test', 'cfg_test_inline_mods',
              'cfg_test_file_mods', 'dead_code', 'unsafe', 'unwrap', 'expect', 'panic', 'pub', 'pub_crate', 'pub_super'):
        print(f'{k}: {acc[k]}')
    print('files>500:', len(acc['big_files']))
    for n, p in sorted(acc['big_files'], reverse=True)[:10]:
        print('  ', n, p)
    for key in ('src', 'test'):
        longs = sorted(acc['long_' + key], reverse=True)
        print(f'fns>20 lines ({key}):', len(longs))
        for n, p, name in longs[:8]:
            print('  ', n, name, p)
        nests = sorted(acc['nest_' + key], reverse=True)
        dist = Counter(min(n, 6) for n, _, _ in nests)
        print(f'nesting dist ({key}, brace depth inside fn body, 6=6+):', dict(sorted(dist.items())))
        for n, p, name in nests[:5]:
            print('  ', n, name, p)


main()
