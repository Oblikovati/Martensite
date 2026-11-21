// SPDX-License-Identifier: GPL-2.0-only
// Twins of ../objects.rs.
#include "parity.hxx"

#include <NCollection_IncAllocator.hxx>
#include <Standard_Transient.hxx>

#include <cstdint>

namespace
{
using parity::Clock;
using parity::Sample;
using parity::since;

class HandleProbe : public Standard_Transient
{
public:
  DEFINE_STANDARD_RTTI_INLINE(HandleProbe, Standard_Transient)
};

Sample handle_create(long long n)
{
  const auto start = Clock::now();
  long long  count = 0;
  for (long long i = 0; i < n; ++i)
  {
    Handle(Standard_Transient) handle = new Standard_Transient();
    parity::keep(handle);
    count += handle->GetRefCount();
  }
  return since(start, n, count);
}

Sample handle_clone(long long n)
{
  Handle(Standard_Transient) handle = new Standard_Transient();
  const long long clones            = 10 * n;
  const auto      start             = Clock::now();
  long long       count             = 0;
  for (long long i = 0; i < clones; ++i)
  {
    Handle(Standard_Transient) copy = handle;
    parity::keep(copy);
    count += copy->GetRefCount();
  }
  return since(start, clones, count);
}

Sample handle_upcast(long long n)
{
  Handle(HandleProbe) owner = new HandleProbe();
  const auto start = Clock::now();
  long long count = 0;
  for (long long i = 0; i < n; ++i)
  {
    parity::keep(owner);
    Handle(Standard_Transient) base = owner;
    parity::keep(base);
    count += base->GetRefCount();
  }
  return since(start, n, count);
}

Sample handle_downcast(long long n)
{
  Handle(Standard_Transient) base = new HandleProbe();
  const auto start = Clock::now();
  long long count = 0;
  for (long long i = 0; i < n; ++i)
  {
    parity::keep(base);
    Handle(HandleProbe) concrete = Handle(HandleProbe)::DownCast(base);
    parity::keep(concrete);
    count += concrete.IsNull() ? 0 : concrete->GetRefCount();
  }
  return since(start, n, count);
}

Sample handle_downcast_ref(long long n)
{
  Handle(Standard_Transient) base = new HandleProbe();
  const auto start = Clock::now();
  long long count = 0;
  for (long long i = 0; i < n; ++i)
  {
    parity::keep(base);
    const auto* concrete = dynamic_cast<const HandleProbe*>(base.get());
    parity::keep(concrete);
    count += concrete != nullptr;
  }
  return since(start, n, count);
}

Sample handle_is_kind(long long n)
{
  Handle(Standard_Transient) base = new HandleProbe();
  const auto& kind = STANDARD_TYPE(Standard_Transient);
  base->DynamicType();
  const auto start = Clock::now();
  long long count = 0;
  for (long long i = 0; i < n; ++i)
  {
    parity::keep(base);
    parity::keep(kind);
    count += base->IsKind(kind);
  }
  return since(start, n, count);
}

// 32-byte values from an incremental arena, kept until the arena is reset.
Sample inc_allocator_value(long long n)
{
  Handle(NCollection_IncAllocator) allocator = new NCollection_IncAllocator(12 * 1024);
  std::vector<uint64_t*>           kept;
  kept.reserve(size_t(n));
  const auto start = Clock::now();
  long long  sum   = 0;
  for (long long i = 0; i < n; ++i)
  {
    uint64_t* value = static_cast<uint64_t*>(allocator->Allocate(4 * sizeof(uint64_t)));
    value[0]        = uint64_t(i & 1);
    value[1] = value[2] = value[3] = 0;
    parity::keep(value);
    sum += (long long)value[0];
    kept.push_back(value);
  }
  kept.clear();
  allocator->Reset(false);
  return since(start, n, sum);
}
} // namespace

void parity::objects(std::vector<Case>& cases)
{
  cases.insert(cases.end(),
               {{"handle_create", handle_create},
                {"handle_clone", handle_clone},
                {"handle_upcast", handle_upcast},
                {"handle_downcast", handle_downcast},
                {"handle_downcast_ref", handle_downcast_ref},
                {"handle_is_kind", handle_is_kind},
                {"inc_allocator_value", inc_allocator_value}});
}
