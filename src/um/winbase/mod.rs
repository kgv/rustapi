pub(crate) use self::lookup_privilege_value::{LookupPrivilegeValue, LookupPrivilegeValueBuilder};

pub fn lookup_privilege_value<'a>() -> LookupPrivilegeValueBuilder<'a, ((), ())> {
    LookupPrivilegeValue::builder()
}

mod lookup_privilege_value;
