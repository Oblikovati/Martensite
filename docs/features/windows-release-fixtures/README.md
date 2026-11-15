# Windows release fixture portability

Bug [#478](https://github.com/Oblikovati/Martensite/issues/478) caused six
build-driver tests to fail in Windows release validation.

Four failures shared a metadata factory that returned `/repo/target`. Windows
considers that path rooted but not drive-qualified absolute. The production
metadata parser correctly rejected it. Both metadata factories now share the
existing native `std::path::absolute` resolution. A regression loads the mocked
metadata through the real parser and checks its output path is absolute.

Two workflow tests split literal LF headings in YAML embedded by `include_str!`.
A Windows CRLF checkout made those headings unavailable; a permissions assertion
also required LF inside its expected block. Both tests now run on explicit LF and
CRLF fixtures, normalize only CRLF, and retain every original comparison-commit,
permission-count and job-scope assertion. Both CRLF cases reproduced locally
before the fix.

Only test fixtures and assertions changed. Production Cargo path validation,
workflow permissions, package versions and release-job execution are unchanged.
Linux validation establishes the LF/CRLF controls; native Windows release
validation remains the platform confirmation for drive-qualified paths.
