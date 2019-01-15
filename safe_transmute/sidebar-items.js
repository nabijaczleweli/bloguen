initSidebarItems({"enum":[["Error","A transmutation error. This type describes possible errors originating from operations in this crate."],["ErrorReason","How the type's size compares to the received byte count and the transmutation function's characteristic."]],"fn":[["guarded_transmute","Transmute a byte slice into a single instance of a `Copy`able type."],["guarded_transmute_bool_pedantic","View a byte slice as a slice of boolean values."],["guarded_transmute_bool_permissive","View a byte slice as a slice of boolean values."],["guarded_transmute_bool_vec_pedantic","Transform a byte vector into a vector of bool."],["guarded_transmute_bool_vec_permissive","Trasform a byte vector into a vector of bool."],["guarded_transmute_many","View a byte slice as a slice of an arbitrary type."],["guarded_transmute_many_pedantic","View a byte slice as a slice of an arbitrary type."],["guarded_transmute_many_permissive","View a byte slice as a slice of an arbitrary type."],["guarded_transmute_pedantic","Transmute a byte slice into a single instance of a `Copy`able type."],["guarded_transmute_pod","Transmute a byte slice into a single instance of a POD."],["guarded_transmute_pod_many","Transmute a byte slice into a single instance of a POD."],["guarded_transmute_pod_many_pedantic","View a byte slice as a slice of POD."],["guarded_transmute_pod_many_permissive","View a byte slice as a slice of a POD type."],["guarded_transmute_pod_pedantic","Transmute a byte slice into a single instance of a POD."],["guarded_transmute_pod_vec","Trasform a byte vector into a vector of POD."],["guarded_transmute_pod_vec_pedantic","Trasform a byte vector into a vector of POD."],["guarded_transmute_pod_vec_permissive","Trasform a byte vector into a vector of POD."],["guarded_transmute_to_bytes","Transmute a single instance of an arbitrary type into a slice of its bytes."],["guarded_transmute_to_bytes_many","Transmute a slice of arbitrary types into a slice of their bytes."],["guarded_transmute_to_bytes_pod","Transmute a single instance of a POD type into a slice of its bytes."],["guarded_transmute_to_bytes_pod_many","Transmute a slice of arbitrary types into a slice of their bytes."],["guarded_transmute_to_bytes_pod_vec","Transmute a vector of POD types into a vector of their bytes, using the same memory buffer as the former."],["guarded_transmute_to_bytes_vec","Transmute a vector of arbitrary types into a vector of their bytes, using the same memory buffer as the former."],["guarded_transmute_vec","Trasform a byte vector into a vector of an arbitrary type."],["guarded_transmute_vec_pedantic","Trasform a byte vector into a vector of an arbitrary type."],["guarded_transmute_vec_permissive","Trasform a byte vector into a vector of an arbitrary type."]],"mod":[["guard","The `guard` module exposes an API for memory boundary checking."],["util","Module containing various utility functions."]],"struct":[["GuardError","A slice boundary guard error, usually created by a `Guard`."]],"trait":[["PodTransmutable","Type that can be non-`unsafe`ly transmuted into"]]});