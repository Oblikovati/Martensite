// SPDX-License-Identifier: GPL-2.0-only
// Twins of ../services.rs.
#include "parity.hxx"

#include <Message_ProgressRange.hxx>
#include <Message_ProgressScope.hxx>
#include <OSD_Parallel.hxx>

#include <algorithm>
#include <atomic>

namespace
{
using parity::Clock;
using parity::Sample;
using parity::since;

// One outer scope of n steps on a null range, with one inner scope per step.
Sample progress_scopes(long long n)
{
  const auto start = Clock::now();
  long long  steps = 0;
  {
    Message_ProgressScope outer(Message_ProgressRange(), "outer", double(n));
    for (long long i = 0; i < n; ++i)
    {
      Message_ProgressScope inner(outer.Next(), "inner", 1.0);
      Message_ProgressRange next = inner.Next();
      parity::keep(next);
      ++steps;
    }
  }
  return since(start, n, steps);
}

// Launch cost: n / 500 parallel loops of 64 items each.
Sample parallel_for_launch64(long long n)
{
  const long long        launches = std::max<long long>(n / 500, 1);
  std::atomic<long long> sum{0};
  const auto             start = Clock::now();
  for (long long l = 0; l < launches; ++l)
    OSD_Parallel::For(0, 64, [&](int i) { sum.fetch_add(i, std::memory_order_relaxed); }, false);
  return since(start, launches, sum.load());
}

// Per-item cost of one parallel loop over n items.
Sample parallel_for_items(long long n)
{
  std::vector<long long> values(size_t(n), 1);
  const auto             start = Clock::now();
  OSD_Parallel::For(0, int(n), [&](int i) { values[size_t(i)] = values[size_t(i)] * 2 + 1; }, false);
  Sample    sample = since(start, n, 0);
  long long sum    = 0;
  for (long long value : values)
    sum += value;
  sample.checksum = sum;
  return sample;
}
} // namespace

void parity::services(std::vector<Case>& cases)
{
  cases.insert(cases.end(),
               {{"progress_scopes", progress_scopes},
                {"parallel_for_launch64", parallel_for_launch64},
                {"parallel_for_items", parallel_for_items}});
}
