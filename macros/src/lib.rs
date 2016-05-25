/// Postgres has a macro called `PG_MODULE_MAGIC` that is supposed
/// to be called within extensions. This generates a bunch
/// of metadata structures that Postgres reads to determine
/// the compatibility of the extension.
///
/// `Pg_magic_func` is the function Postgres will call
/// to check compatibility with memcmp, so there can't be
/// any alignment differences.
///
/// Usage:
///
/// ```notrust
/// pg_module!(90500)
/// ```
#[macro_export]
macro_rules! pg_module {
    (version: $vers:expr) => {
        static mut Pg_magic_data: postgres_extension::Pg_magic_struct =
            postgres_extension::Pg_magic_struct {
                len: 0 as c_int,
                version: $vers,
                funcmaxargs: 100,
                indexmaxkeys: 32,
                nameddatalen: 64,
                float4byval: 1,
                float8byval: 1
            };


        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern fn Pg_magic_func() -> &'static postgres_extension::Pg_magic_struct {
            use std::mem::size_of;
            use libc::{c_int};

            unsafe {
                Pg_magic_data = postgres_extension::Pg_magic_struct {
                    len: size_of::<postgres_extension::Pg_magic_struct>() as c_int,
                    version: $vers / 100,
                    funcmaxargs: 100,
                    indexmaxkeys: 32,
                    nameddatalen: 64,
                    float4byval: 1,
                    float8byval: 1
                };

                &Pg_magic_data
            }
        }
    }
}
