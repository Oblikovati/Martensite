// SPDX-License-Identifier: GPL-2.0-only
// OCCT twins of the Rust cases in ../ (#583). Each case does the same work as the Rust case
// of the same name and returns the same checksum.
#ifndef MARTENSITE_OCCT_PARITY_HXX
#define MARTENSITE_OCCT_PARITY_HXX

#include <chrono>
#include <vector>

namespace parity
{
//! One timed run: the measured nanoseconds, the operation count and the work checksum.
struct Sample
{
  double    nanos;
  long long ops;
  long long checksum;
};

using Clock = std::chrono::steady_clock;

//! Closes a timed section that started at `start`.
inline Sample since(Clock::time_point start, long long ops, long long checksum)
{
  const auto elapsed = std::chrono::duration<double, std::nano>(Clock::now() - start).count();
  return {elapsed, ops, checksum};
}

//! A named case; `run` receives the item count `n` and does its work once.
struct Case
{
  const char* name;
  Sample (*run)(long long n);
};

//! Keeps a value observable so the compiler cannot drop the work that produced it.
template <typename T>
inline void keep(T& value)
{
  asm volatile("" : : "r"(&value) : "memory");
}

void collections(std::vector<Case>& cases);
void objects(std::vector<Case>& cases);
void text(std::vector<Case>& cases);
void services(std::vector<Case>& cases);
} // namespace parity

#endif
