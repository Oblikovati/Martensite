// SPDX-License-Identifier: GPL-2.0-only
// Runs the OCCT twins of ../main.rs with the same arguments and output format:
// `--bench` measures (1,000,000 items, median of 7), otherwise every case runs once at a
// small size; `--filter <text>` keeps the cases whose name contains the text.
#include "parity.hxx"

#include <algorithm>
#include <cstdio>
#include <cstring>
#include <string>

namespace
{
struct Mode
{
  long long   items   = 1000;
  int         repeats = 1;
  std::string filter;
};

Mode parse(int argc, char** argv)
{
  Mode mode;
  for (int i = 1; i < argc; ++i)
  {
    if (std::strcmp(argv[i], "--bench") == 0)
    {
      mode.items   = 1000000;
      mode.repeats = 7;
    }
    else if (std::strcmp(argv[i], "--filter") == 0 && i + 1 < argc)
    {
      mode.filter = argv[++i];
    }
  }
  return mode;
}

void report(const parity::Case& item, const Mode& mode)
{
  std::vector<double> times;
  long long           checksum = 0;
  for (int r = 0; r < mode.repeats; ++r)
  {
    const parity::Sample sample = item.run(mode.items);
    times.push_back(sample.nanos / double(sample.ops));
    checksum = sample.checksum;
  }
  std::sort(times.begin(), times.end());
  std::printf("%s\t%.3f\t%lld\n", item.name, times[times.size() / 2], checksum);
}
} // namespace

int main(int argc, char** argv)
{
  const Mode                mode = parse(argc, argv);
  std::vector<parity::Case> cases;
  parity::collections(cases);
  parity::objects(cases);
  parity::text(cases);
  parity::services(cases);
  for (const parity::Case& item : cases)
  {
    if (mode.filter.empty() || std::strstr(item.name, mode.filter.c_str()) != nullptr)
    {
      report(item, mode);
    }
  }
  return 0;
}
