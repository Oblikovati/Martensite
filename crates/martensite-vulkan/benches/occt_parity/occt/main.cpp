// SPDX-License-Identifier: GPL-2.0-only
// Runs the OCCT twins of ../linux.rs with the same arguments and output format:
// `--bench` measures (100 frames, median of 7), otherwise every case draws two frames;
// `--filter <text>` keeps the cases whose name contains the text, and `--frames <n>` changes the
// frames (operations) of one timed run.
#include "parity.hxx"

#include <X11/Xlib.h>

#include <algorithm>
#include <cstdio>
#include <cstdlib>
#include <cstring>

namespace
{
parity::Mode parse(int argc, char** argv)
{
  parity::Mode mode;
  int          frames = 0;
  for (int i = 1; i < argc; ++i)
  {
    if (std::strcmp(argv[i], "--bench") == 0)
    {
      mode.frames  = 100;
      mode.repeats = 7;
      mode.warmup  = 10;
    }
    else if (std::strcmp(argv[i], "--filter") == 0 && i + 1 < argc)
    {
      mode.filter = argv[++i];
    }
    else if (std::strcmp(argv[i], "--frames") == 0 && i + 1 < argc)
    {
      frames = std::atoi(argv[++i]);
      if (frames <= 0)
      {
        std::fprintf(stderr, "received --frames %s; expected a positive frame count\n", argv[i]);
        std::exit(2);
      }
    }
  }
  if (frames > 0)
  {
    mode.frames = frames;
  }
  mode.warmup = std::min(mode.warmup, mode.frames);
  return mode;
}

void report(const parity::Bench& bench, const parity::Case& item, const parity::Mode& mode)
{
  parity::Measured measured = item.run(bench, mode);
  std::sort(measured.nanos.begin(), measured.nanos.end());
  std::printf("%s\t%.3f\t%lld\n", item.name, measured.nanos[measured.nanos.size() / 2], measured.checksum);
  std::fflush(stdout);
}
} // namespace

int main(int argc, char** argv)
{
  XInitThreads();
  const parity::Mode mode = parse(argc, argv);
  parity::Bench      bench;
  bench.driver = parity::new_driver(new Aspect_DisplayConnection());
  std::vector<parity::Case> cases;
  parity::startup(cases);
  parity::frames(cases);
  parity::ais(cases);
  parity::transfer(cases);
  for (const parity::Case& item : cases)
  {
    if (mode.filter.empty() || std::strstr(item.name, mode.filter.c_str()) != nullptr)
    {
      report(bench, item, mode);
    }
  }
  return 0;
}
