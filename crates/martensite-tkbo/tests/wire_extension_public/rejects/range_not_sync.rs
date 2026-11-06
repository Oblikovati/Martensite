use martensite_tkernel::message::ProgressRange;

fn require_shared_range<T: Sync>() {
    let drop_owned: fn(T) = std::mem::drop::<T>;
    std::hint::black_box(drop_owned);
}

fn main() {
    require_shared_range::<ProgressRange<'static>>();
    std::hint::black_box("range must remain owned per task");
}
