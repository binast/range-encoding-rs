#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]

extern crate libc;
extern "C" {
    pub type __sFILEX;
    #[no_mangle]
    static mut __stdinp: *mut FILE;
    #[no_mangle]
    static mut __stdoutp: *mut FILE;
    #[no_mangle]
    static mut __stderrp: *mut FILE;
    #[no_mangle]
    fn __swbuf(_: libc::c_int, _: *mut FILE) -> libc::c_int;
    #[no_mangle]
    static sys_nerr: libc::c_int;
    #[no_mangle]
    static sys_errlist: [*const libc::c_char; 0];
    #[no_mangle]
    static mut __mb_cur_max: libc::c_int;
    #[no_mangle]
    fn free(_: *mut libc::c_void) -> ();
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    static mut suboptarg: *mut libc::c_char;
    /* *
   @file arch.h
   @brief Various architecture definitions for CELT
*/
    #[no_mangle]
    fn celt_fatal(str: *const libc::c_char, file: *const libc::c_char,
                  line: libc::c_int) -> !;
    #[no_mangle]
    fn __sincosf_stret(_: libc::c_float) -> __float2;
    #[no_mangle]
    fn __sincos_stret(_: libc::c_double) -> __double2;
    #[no_mangle]
    fn __sincospif_stret(_: libc::c_float) -> __float2;
    #[no_mangle]
    fn __sincospi_stret(_: libc::c_double) -> __double2;
    #[no_mangle]
    static mut signgam: libc::c_int;
    #[no_mangle]
    static SMALL_DIV_TABLE: [opus_uint32; 129];
}
pub type size_t = libc::c_ulong;
pub type int32_t = libc::c_int;
pub type uint32_t = libc::c_uint;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_longlong;
pub type __uint64_t = libc::c_ulonglong;
pub type __darwin_off_t = __int64_t;
pub type opus_int32 = int32_t;
pub type opus_uint32 = uint32_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed {
    __f: libc::c_float,
    __u: libc::c_uint,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_0 {
    __ld: libc::c_double,
    __p: unnamed_2,
}
pub type ec_window = opus_uint32;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct __float2 {
    pub __sinval: libc::c_float,
    pub __cosval: libc::c_float,
}
pub type fpos_t = __darwin_off_t;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut libc::c_uchar,
    pub _size: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut libc::c_uchar,
    pub _r: libc::c_int,
    pub _w: libc::c_int,
    pub _flags: libc::c_short,
    pub _file: libc::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: libc::c_int,
    pub _cookie: *mut libc::c_void,
    pub _close: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                           -> libc::c_int>,
    pub _read: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                           _: *mut libc::c_char,
                                           _: libc::c_int) -> libc::c_int>,
    pub _seek: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: fpos_t,
                                           _: libc::c_int) -> fpos_t>,
    pub _write: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                            _: *const libc::c_char,
                                            _: libc::c_int) -> libc::c_int>,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: libc::c_int,
    pub _ubuf: [libc::c_uchar; 3],
    pub _nbuf: [libc::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: libc::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct __double2 {
    pub __sinval: libc::c_double,
    pub __cosval: libc::c_double,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_1 {
    __f: libc::c_double,
    __u: libc::c_ulonglong,
}
pub type ec_ctx = ec_ctx_0;
pub type ec_dec = ec_ctx_0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ec_ctx_0 {
    pub buf: *mut libc::c_uchar,
    pub storage: opus_uint32,
    pub end_offs: opus_uint32,
    pub end_window: ec_window,
    pub nend_bits: libc::c_int,
    pub nbits_total: libc::c_int,
    pub offs: opus_uint32,
    pub rng: opus_uint32,
    pub val: opus_uint32,
    pub ext: opus_uint32,
    pub rem: libc::c_int,
    pub error: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_2 {
    pub __m: libc::c_ulonglong,
    pub __sexp: libc::c_ushort,
}
unsafe extern "C" fn __sputc(mut _c: libc::c_int, mut _p: *mut FILE)
 -> libc::c_int {
    (*_p)._w -= 1;
    if (*_p)._w >= 0i32 ||
           (*_p)._w >= (*_p)._lbfsize &&
               _c as libc::c_char as libc::c_int != '\n' as i32 {
        let fresh0 = (*_p)._p;
        (*_p)._p = (*_p)._p.offset(1);
        *fresh0 = _c as libc::c_uchar;
        return *fresh0 as libc::c_int
    } else { return __swbuf(_c, _p) };
}
unsafe extern "C" fn _OSSwapInt16(mut _data: __uint16_t) -> __uint16_t {
    return ((_data as libc::c_int) << 8i32 | _data as libc::c_int >> 8i32) as
               __uint16_t;
}
unsafe extern "C" fn _OSSwapInt32(mut _data: __uint32_t) -> __uint32_t {
    return _data.swap_bytes();
}
unsafe extern "C" fn _OSSwapInt64(mut _data: __uint64_t) -> __uint64_t {
    return _data.swap_bytes();
}
/* * Opus wrapper for malloc(). To do your own dynamic allocation, all you need to do is replace this function and opus_free */
unsafe extern "C" fn opus_alloc(mut size: size_t) -> *mut libc::c_void {
    return malloc(size);
}
/* * Same as celt_alloc(), except that the area is only needed inside a CELT call (might cause problem with wideband though) */
unsafe extern "C" fn opus_alloc_scratch(mut size: size_t)
 -> *mut libc::c_void {
    return opus_alloc(size);
}
/* * Opus wrapper for free(). To do your own dynamic allocation, all you need to do is replace this function and opus_alloc */
unsafe extern "C" fn opus_free(mut ptr: *mut libc::c_void) -> () {
    free(ptr);
}
unsafe extern "C" fn __inline_isfinitef(mut __x: libc::c_float)
 -> libc::c_int {
    return (__x == __x && __x.abs() != ::std::f32::INFINITY) as libc::c_int;
}
unsafe extern "C" fn __inline_isfinited(mut __x: libc::c_double)
 -> libc::c_int {
    return (__x == __x && __x.abs() != ::std::f64::INFINITY) as libc::c_int;
}
unsafe extern "C" fn __inline_isfinitel(mut __x: libc::c_double)
 -> libc::c_int {
    return (__x == __x && __x.abs() != ::std::f64::INFINITY) as libc::c_int;
}
unsafe extern "C" fn __inline_isinff(mut __x: libc::c_float) -> libc::c_int {
    return (__x.abs() == ::std::f32::INFINITY) as libc::c_int;
}
unsafe extern "C" fn __inline_isinfd(mut __x: libc::c_double) -> libc::c_int {
    return (__x.abs() == ::std::f64::INFINITY) as libc::c_int;
}
unsafe extern "C" fn __inline_isinfl(mut __x: libc::c_double) -> libc::c_int {
    return (__x.abs() == ::std::f64::INFINITY) as libc::c_int;
}
unsafe extern "C" fn __inline_isnanf(mut __x: libc::c_float) -> libc::c_int {
    return (__x != __x) as libc::c_int;
}
unsafe extern "C" fn __inline_isnand(mut __x: libc::c_double) -> libc::c_int {
    return (__x != __x) as libc::c_int;
}
unsafe extern "C" fn __inline_isnanl(mut __x: libc::c_double) -> libc::c_int {
    return (__x != __x) as libc::c_int;
}
unsafe extern "C" fn __inline_isnormalf(mut __x: libc::c_float)
 -> libc::c_int {
    return (0 != __inline_isfinitef(__x) &&
                __x.abs() >= 1.1754943508222876e-38f32) as libc::c_int;
}
unsafe extern "C" fn __inline_isnormald(mut __x: libc::c_double)
 -> libc::c_int {
    return (0 != __inline_isfinited(__x) &&
                __x.abs() >= 2.2250738585072014e-308f64) as libc::c_int;
}
unsafe extern "C" fn __inline_isnormall(mut __x: libc::c_double)
 -> libc::c_int {
    return (0 != __inline_isfinitel(__x) && __x.abs() >= 0.0f64) as
               libc::c_int;
}
unsafe extern "C" fn __inline_signbitf(mut __x: libc::c_float)
 -> libc::c_int {
    let mut __u: unnamed = unnamed{__f: 0.,};
    __u.__f = __x;
    return (__u.__u >> 31i32) as libc::c_int;
}
unsafe extern "C" fn __inline_signbitd(mut __x: libc::c_double)
 -> libc::c_int {
    let mut __u: unnamed_1 = unnamed_1{__f: 0.,};
    __u.__f = __x;
    return (__u.__u >> 63i32) as libc::c_int;
}
unsafe extern "C" fn __inline_signbitl(mut __x: libc::c_double)
 -> libc::c_int {
    let mut __u: unnamed_0 = unnamed_0{__ld: 0.,};
    __u.__ld = __x;
    return __u.__p.__sexp as libc::c_int >> 15i32;
}
unsafe extern "C" fn __sincosf(mut __x: libc::c_float,
                               mut __sinp: *mut libc::c_float,
                               mut __cosp: *mut libc::c_float) -> () {
    let __stret: __float2 = __sincosf_stret(__x);
    *__sinp = __stret.__sinval;
    *__cosp = __stret.__cosval;
}
unsafe extern "C" fn __sincos(mut __x: libc::c_double,
                              mut __sinp: *mut libc::c_double,
                              mut __cosp: *mut libc::c_double) -> () {
    let __stret: __double2 = __sincos_stret(__x);
    *__sinp = __stret.__sinval;
    *__cosp = __stret.__cosval;
}
unsafe extern "C" fn __sincospif(mut __x: libc::c_float,
                                 mut __sinp: *mut libc::c_float,
                                 mut __cosp: *mut libc::c_float) -> () {
    let __stret: __float2 = __sincospif_stret(__x);
    *__sinp = __stret.__sinval;
    *__cosp = __stret.__cosval;
}
unsafe extern "C" fn __sincospi(mut __x: libc::c_double,
                                mut __sinp: *mut libc::c_double,
                                mut __cosp: *mut libc::c_double) -> () {
    let __stret: __double2 = __sincospi_stret(__x);
    *__sinp = __stret.__sinval;
    *__cosp = __stret.__cosval;
}
unsafe extern "C" fn ec_range_bytes(mut _this: *mut ec_ctx) -> opus_uint32 {
    return (*_this).offs;
}
unsafe extern "C" fn ec_get_buffer(mut _this: *mut ec_ctx)
 -> *mut libc::c_uchar {
    return (*_this).buf;
}
pub unsafe extern "C" fn ec_get_error(mut _this: *mut ec_ctx) -> libc::c_int {
    return (*_this).error;
}
unsafe extern "C" fn ec_tell(mut _this: *mut ec_ctx) -> libc::c_int {
    return (*_this).nbits_total -
               (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong as
                    libc::c_int * 8i32 - (*_this).rng.leading_zeros() as i32);
}
unsafe extern "C" fn celt_udiv(mut n: opus_uint32, mut d: opus_uint32)
 -> opus_uint32 {
    return n.wrapping_div(d);
}
unsafe extern "C" fn celt_sudiv(mut n: opus_int32, mut d: opus_int32)
 -> opus_int32 {
    return n / d;
}
#[no_mangle]
pub unsafe extern "C" fn ec_dec_init(mut _this: *mut ec_dec,
                                     mut _buf: *mut libc::c_uchar,
                                     mut _storage: opus_uint32) -> () {
    (*_this).buf = _buf;
    (*_this).storage = _storage;
    (*_this).end_offs = 0i32 as opus_uint32;
    (*_this).end_window = 0i32 as ec_window;
    (*_this).nend_bits = 0i32;
    (*_this).nbits_total =
        32i32 + 1i32 - (32i32 - ((32i32 - 2i32) % 8i32 + 1i32)) / 8i32 * 8i32;
    (*_this).offs = 0i32 as opus_uint32;
    (*_this).rng = 1u32 << (32i32 - 2i32) % 8i32 + 1i32;
    (*_this).rem = ec_read_byte(_this);
    (*_this).val =
        (*_this).rng.wrapping_sub(1i32 as
                                      libc::c_uint).wrapping_sub(((*_this).rem
                                                                      >>
                                                                      8i32 -
                                                                          ((32i32
                                                                                -
                                                                                2i32)
                                                                               %
                                                                               8i32
                                                                               +
                                                                               1i32))
                                                                     as
                                                                     libc::c_uint);
    (*_this).error = 0i32;
    ec_dec_normalize(_this);
}
unsafe extern "C" fn ec_dec_normalize(mut _this: *mut ec_dec) -> () {
    while (*_this).rng <= 1u32 << 32i32 - 1i32 >> 8i32 {
        let mut sym: libc::c_int = 0;
        (*_this).nbits_total += 8i32;
        (*_this).rng <<= 8i32;
        sym = (*_this).rem;
        (*_this).rem = ec_read_byte(_this);
        sym =
            (sym << 8i32 | (*_this).rem) >>
                8i32 - ((32i32 - 2i32) % 8i32 + 1i32);
        (*_this).val =
            ((*_this).val <<
                 8i32).wrapping_add((1u32 <<
                                         8i32).wrapping_sub(1i32 as
                                                                libc::c_uint)
                                        & !sym as libc::c_uint) &
                (1u32 << 32i32 - 1i32).wrapping_sub(1i32 as libc::c_uint)
    };
}
unsafe extern "C" fn ec_read_byte(mut _this: *mut ec_dec) -> libc::c_int {
    return if (*_this).offs < (*_this).storage {
               let fresh1 = (*_this).offs;
               (*_this).offs = (*_this).offs.wrapping_add(1);
               *(*_this).buf.offset(fresh1 as isize) as libc::c_int
           } else { 0i32 };
}
#[no_mangle]
pub unsafe extern "C" fn ec_decode(mut _this: *mut ec_dec,
                                   mut _ft: libc::c_uint) -> libc::c_uint {
    let mut s: libc::c_uint = 0;
    (*_this).ext = celt_udiv((*_this).rng, _ft);
    s = (*_this).val.wrapping_div((*_this).ext);
    return _ft.wrapping_sub(s.wrapping_add(1i32 as
                                               libc::c_uint).wrapping_add(_ft.wrapping_sub(s.wrapping_add(1i32
                                                                                                              as
                                                                                                              libc::c_uint))
                                                                              &
                                                                              -((_ft
                                                                                     <
                                                                                     s.wrapping_add(1i32
                                                                                                        as
                                                                                                        libc::c_uint))
                                                                                    as
                                                                                    libc::c_int)
                                                                                  as
                                                                                  libc::c_uint));
}
#[no_mangle]
pub unsafe extern "C" fn ec_decode_bin(mut _this: *mut ec_dec,
                                       mut _bits: libc::c_uint)
 -> libc::c_uint {
    let mut s: libc::c_uint = 0;
    (*_this).ext = (*_this).rng >> _bits;
    s = (*_this).val.wrapping_div((*_this).ext);
    return (1u32 <<
                _bits).wrapping_sub(s.wrapping_add(1u32).wrapping_add((1u32 <<
                                                                           _bits).wrapping_sub(s.wrapping_add(1u32))
                                                                          &
                                                                          -((1u32
                                                                                 <<
                                                                                 _bits
                                                                                 <
                                                                                 s.wrapping_add(1u32))
                                                                                as
                                                                                libc::c_int)
                                                                              as
                                                                              libc::c_uint));
}
#[no_mangle]
pub unsafe extern "C" fn ec_dec_update(mut _this: *mut ec_dec,
                                       mut _fl: libc::c_uint,
                                       mut _fh: libc::c_uint,
                                       mut _ft: libc::c_uint) -> () {
    let mut s: opus_uint32 = 0;
    s = (*_this).ext.wrapping_mul(_ft.wrapping_sub(_fh));
    (*_this).val =
        ((*_this).val as libc::c_uint).wrapping_sub(s) as opus_uint32 as
            opus_uint32;
    (*_this).rng =
        if _fl > 0i32 as libc::c_uint {
            (*_this).ext.wrapping_mul(_fh.wrapping_sub(_fl))
        } else { (*_this).rng.wrapping_sub(s) };
    ec_dec_normalize(_this);
}
#[no_mangle]
pub unsafe extern "C" fn ec_dec_bit_logp(mut _this: *mut ec_dec,
                                         mut _logp: libc::c_uint)
 -> libc::c_int {
    let mut r: opus_uint32 = 0;
    let mut d: opus_uint32 = 0;
    let mut s: opus_uint32 = 0;
    let mut ret: libc::c_int = 0;
    r = (*_this).rng;
    d = (*_this).val;
    s = r >> _logp;
    ret = (d < s) as libc::c_int;
    if 0 == ret { (*_this).val = d.wrapping_sub(s) }
    (*_this).rng = if 0 != ret { s } else { r.wrapping_sub(s) };
    ec_dec_normalize(_this);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ec_dec_icdf(mut _this: *mut ec_dec,
                                     mut _icdf: *const libc::c_uchar,
                                     mut _ftb: libc::c_uint) -> libc::c_int {
    let mut r: opus_uint32 = 0;
    let mut d: opus_uint32 = 0;
    let mut s: opus_uint32 = 0;
    let mut t: opus_uint32 = 0;
    let mut ret: libc::c_int = 0;
    s = (*_this).rng;
    d = (*_this).val;
    r = s >> _ftb;
    ret = -1i32;
    loop  {
        t = s;
        ret += 1;
        s = r.wrapping_mul(*_icdf.offset(ret as isize) as libc::c_uint);
        if !(d < s) { break ; }
    }
    (*_this).val = d.wrapping_sub(s);
    (*_this).rng = t.wrapping_sub(s);
    ec_dec_normalize(_this);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ec_dec_uint(mut _this: *mut ec_dec,
                                     mut _ft: opus_uint32) -> opus_uint32 {
    let mut ft: libc::c_uint = 0;
    let mut s: libc::c_uint = 0;
    let mut ftb: libc::c_int = 0;
    if !(_ft > 1i32 as libc::c_uint) {
        celt_fatal((*::std::mem::transmute::<&[u8; 24],
                                             &mut [libc::c_char; 24]>(b"assertion failed: _ft>1\x00")).as_mut_ptr(),
                   (*::std::mem::transmute::<&[u8; 14],
                                             &mut [libc::c_char; 14]>(b"celt/entdec.c\x00")).as_mut_ptr(),
                   203i32);
    } else {
        _ft = _ft.wrapping_sub(1);
        ftb =
            ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong as
                libc::c_int * 8i32 - _ft.leading_zeros() as i32;
        if ftb > 8i32 {
            let mut t: opus_uint32 = 0;
            ftb -= 8i32;
            ft = (_ft >> ftb).wrapping_add(1i32 as libc::c_uint);
            s = ec_decode(_this, ft);
            ec_dec_update(_this, s, s.wrapping_add(1i32 as libc::c_uint), ft);
            t = s << ftb | ec_dec_bits(_this, ftb as libc::c_uint);
            if t <= _ft {
                return t
            } else { (*_this).error = 1i32; return _ft }
        } else {
            _ft = _ft.wrapping_add(1);
            s = ec_decode(_this, _ft);
            ec_dec_update(_this, s, s.wrapping_add(1i32 as libc::c_uint),
                          _ft);
            return s
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ec_dec_bits(mut _this: *mut ec_dec,
                                     mut _bits: libc::c_uint) -> opus_uint32 {
    let mut window: ec_window = 0;
    let mut available: libc::c_int = 0;
    let mut ret: opus_uint32 = 0;
    window = (*_this).end_window;
    available = (*_this).nend_bits;
    if (available as libc::c_uint) < _bits {
        loop  {
            window |=
                (ec_read_byte_from_end(_this) as ec_window) << available;
            available += 8i32;
            if !(available <=
                     ::std::mem::size_of::<ec_window>() as libc::c_ulong as
                         libc::c_int * 8i32 - 8i32) {
                break ;
            }
        }
    }
    ret = window & ((1i32 as opus_uint32) << _bits).wrapping_sub(1u32);
    window >>= _bits;
    available =
        (available as libc::c_uint).wrapping_sub(_bits) as libc::c_int as
            libc::c_int;
    (*_this).end_window = window;
    (*_this).nend_bits = available;
    (*_this).nbits_total =
        ((*_this).nbits_total as libc::c_uint).wrapping_add(_bits) as
            libc::c_int as libc::c_int;
    return ret;
}
unsafe extern "C" fn ec_read_byte_from_end(mut _this: *mut ec_dec)
 -> libc::c_int {
    return if (*_this).end_offs < (*_this).storage {
               (*_this).end_offs = (*_this).end_offs.wrapping_add(1);
               *(*_this).buf.offset((*_this).storage.wrapping_sub((*_this).end_offs)
                                        as isize) as libc::c_int
           } else { 0i32 };
}
