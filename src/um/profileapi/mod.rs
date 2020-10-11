pub(crate) use self::query_performance_counter::{
    QueryPerformanceCounter, QueryPerformanceCounterBuilder,
};

pub fn query_performance_counter() -> QueryPerformanceCounterBuilder<()> {
    QueryPerformanceCounter::builder()
}

mod query_performance_counter;
