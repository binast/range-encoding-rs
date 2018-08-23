#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]

extern crate libc;
extern "C" {
    pub type __sFILEX;
}
pub type int32_t = libc::c_int;
pub type uint32_t = libc::c_uint;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_longlong;
pub type __uint64_t = libc::c_ulonglong;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_off_t = __int64_t;
pub type opus_int32 = int32_t;
pub type opus_uint32 = uint32_t;
pub type size_t = __darwin_size_t;
#[derive ( Clone, Debug )]
#[repr(C)]
pub struct ec_ctx {
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
#[repr ( C )]
pub union unnamed {
    __f: libc::c_double,
    __u: libc::c_ulonglong,
}
pub type ec_enc = ec_ctx;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_0 {
    pub __m: libc::c_ulonglong,
    pub __sexp: libc::c_ushort,
}
pub type ec_window = opus_uint32;
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
#[repr ( C )]
pub union unnamed_1 {
    __ld: libc::c_double,
    __p: unnamed_0,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct __double2 {
    pub __sinval: libc::c_double,
    pub __cosval: libc::c_double,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct __float2 {
    pub __sinval: libc::c_float,
    pub __cosval: libc::c_float,
}
pub type ec_ctx_0 = ec_ctx;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_2 {
    __f: libc::c_float,
    __u: libc::c_uint,
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
    let mut __u: unnamed_2 = unnamed_2{__f: 0.,};
    __u.__f = __x;
    return (__u.__u >> 31i32) as libc::c_int;
}
unsafe extern "C" fn __inline_signbitd(mut __x: libc::c_double)
 -> libc::c_int {
    let mut __u: unnamed = unnamed{__f: 0.,};
    __u.__f = __x;
    return (__u.__u >> 63i32) as libc::c_int;
}
unsafe extern "C" fn __inline_signbitl(mut __x: libc::c_double)
 -> libc::c_int {
    let mut __u: unnamed_1 = unnamed_1{__ld: 0.,};
    __u.__ld = __x;
    return __u.__p.__sexp as libc::c_int >> 15i32;
}

pub unsafe extern "C" fn ec_range_bytes(mut _this: *mut ec_ctx_0) -> opus_uint32 {
    return (*_this).offs;
}
unsafe extern "C" fn ec_get_buffer(mut _this: *mut ec_ctx_0)
 -> *mut libc::c_uchar {
    return (*_this).buf;
}
pub unsafe extern "C" fn ec_get_error(mut _this: *mut ec_ctx_0) -> libc::c_int {
    return (*_this).error;
}
unsafe extern "C" fn ec_tell(mut _this: *mut ec_ctx_0) -> libc::c_int {
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

pub unsafe extern "C" fn ec_enc_init(mut _this: *mut ec_enc,
                                     mut _buf: *mut libc::c_uchar,
                                     mut _size: opus_uint32) -> () {
    (*_this).buf = _buf;
    (*_this).end_offs = 0i32 as opus_uint32;
    (*_this).end_window = 0i32 as ec_window;
    (*_this).nend_bits = 0i32;
    (*_this).nbits_total = 32i32 + 1i32;
    (*_this).offs = 0i32 as opus_uint32;
    (*_this).rng = 1u32 << 32i32 - 1i32;
    (*_this).rem = -1i32;
    (*_this).val = 0i32 as opus_uint32;
    (*_this).ext = 0i32 as opus_uint32;
    (*_this).storage = _size;
    (*_this).error = 0i32;
}

pub unsafe extern "C" fn ec_encode(mut _this: *mut ec_enc,
                                   mut _fl: libc::c_uint,
                                   mut _fh: libc::c_uint,
                                   mut _ft: libc::c_uint) -> () {
    let mut r: opus_uint32 = 0;
    r = celt_udiv((*_this).rng, _ft);
    if _fl > 0i32 as libc::c_uint {
        (*_this).val =
            ((*_this).val as
                 libc::c_uint).wrapping_add((*_this).rng.wrapping_sub(r.wrapping_mul(_ft.wrapping_sub(_fl))))
                as opus_uint32 as opus_uint32;
        (*_this).rng = r.wrapping_mul(_fh.wrapping_sub(_fl))
    } else {
        (*_this).rng =
            ((*_this).rng as
                 libc::c_uint).wrapping_sub(r.wrapping_mul(_ft.wrapping_sub(_fh)))
                as opus_uint32 as opus_uint32
    }
    ec_enc_normalize(_this);
}
unsafe extern "C" fn ec_enc_normalize(mut _this: *mut ec_enc) -> () {
    while (*_this).rng <= 1u32 << 32i32 - 1i32 >> 8i32 {
        ec_enc_carry_out(_this,
                         ((*_this).val >> 32i32 - 8i32 - 1i32) as
                             libc::c_int);
        (*_this).val =
            (*_this).val << 8i32 &
                (1u32 << 32i32 - 1i32).wrapping_sub(1i32 as libc::c_uint);
        (*_this).rng <<= 8i32;
        (*_this).nbits_total += 8i32
    };
}
unsafe extern "C" fn ec_enc_carry_out(mut _this: *mut ec_enc,
                                      mut _c: libc::c_int) -> () {
    if _c as libc::c_uint != (1u32 << 8i32).wrapping_sub(1i32 as libc::c_uint)
       {
        let mut carry: libc::c_int = 0;
        carry = _c >> 8i32;
        if (*_this).rem >= 0i32 {
            (*_this).error |=
                ec_write_byte(_this, ((*_this).rem + carry) as libc::c_uint)
        }
        if (*_this).ext > 0i32 as libc::c_uint {
            let mut sym: libc::c_uint = 0;
            sym =
                (1u32 <<
                     8i32).wrapping_sub(1i32 as
                                            libc::c_uint).wrapping_add(carry
                                                                           as
                                                                           libc::c_uint)
                    & (1u32 << 8i32).wrapping_sub(1i32 as libc::c_uint);
            loop  {
                (*_this).error |= ec_write_byte(_this, sym);
                (*_this).ext = (*_this).ext.wrapping_sub(1);
                if !((*_this).ext > 0i32 as libc::c_uint) { break ; }
            }
        }
        (*_this).rem =
            (_c as libc::c_uint &
                 (1u32 << 8i32).wrapping_sub(1i32 as libc::c_uint)) as
                libc::c_int
    } else { (*_this).ext = (*_this).ext.wrapping_add(1) };
}
unsafe extern "C" fn ec_write_byte(mut _this: *mut ec_enc,
                                   mut _value: libc::c_uint) -> libc::c_int {
    if (*_this).offs.wrapping_add((*_this).end_offs) >= (*_this).storage {
        return -1i32
    } else {
        let fresh1 = (*_this).offs;
        (*_this).offs = (*_this).offs.wrapping_add(1);
        *(*_this).buf.offset(fresh1 as isize) = _value as libc::c_uchar;
        return 0i32
    };
}

pub unsafe extern "C" fn ec_encode_bin(mut _this: *mut ec_enc,
                                       mut _fl: libc::c_uint,
                                       mut _fh: libc::c_uint,
                                       mut _bits: libc::c_uint) -> () {
    let mut r: opus_uint32 = 0;
    r = (*_this).rng >> _bits;
    if _fl > 0i32 as libc::c_uint {
        (*_this).val =
            ((*_this).val as
                 libc::c_uint).wrapping_add((*_this).rng.wrapping_sub(r.wrapping_mul((1u32
                                                                                          <<
                                                                                          _bits).wrapping_sub(_fl))))
                as opus_uint32 as opus_uint32;
        (*_this).rng = r.wrapping_mul(_fh.wrapping_sub(_fl))
    } else {
        (*_this).rng =
            ((*_this).rng as
                 libc::c_uint).wrapping_sub(r.wrapping_mul((1u32 <<
                                                                _bits).wrapping_sub(_fh)))
                as opus_uint32 as opus_uint32
    }
    ec_enc_normalize(_this);
}

pub unsafe extern "C" fn ec_enc_bit_logp(mut _this: *mut ec_enc,
                                         mut _val: libc::c_int,
                                         mut _logp: libc::c_uint) -> () {
    let mut r: opus_uint32 = 0;
    let mut s: opus_uint32 = 0;
    let mut l: opus_uint32 = 0;
    r = (*_this).rng;
    l = (*_this).val;
    s = r >> _logp;
    r = (r as libc::c_uint).wrapping_sub(s) as opus_uint32 as opus_uint32;
    if 0 != _val { (*_this).val = l.wrapping_add(r) }
    (*_this).rng = if 0 != _val { s } else { r };
    ec_enc_normalize(_this);
}

pub unsafe extern "C" fn ec_enc_icdf(mut _this: *mut ec_enc,
                                     mut _s: libc::c_int,
                                     mut _icdf: *const libc::c_uchar,
                                     mut _ftb: libc::c_uint) -> () {
    let mut r: opus_uint32 = 0;
    r = (*_this).rng >> _ftb;
    if _s > 0i32 {
        (*_this).val =
            ((*_this).val as
                 libc::c_uint).wrapping_add((*_this).rng.wrapping_sub(r.wrapping_mul(*_icdf.offset((_s
                                                                                                        -
                                                                                                        1i32)
                                                                                                       as
                                                                                                       isize)
                                                                                         as
                                                                                         libc::c_uint)))
                as opus_uint32 as opus_uint32;
        (*_this).rng =
            r.wrapping_mul((*_icdf.offset((_s - 1i32) as isize) as libc::c_int
                                - *_icdf.offset(_s as isize) as libc::c_int)
                               as libc::c_uint)
    } else {
        (*_this).rng =
            ((*_this).rng as
                 libc::c_uint).wrapping_sub(r.wrapping_mul(*_icdf.offset(_s as
                                                                             isize)
                                                               as
                                                               libc::c_uint))
                as opus_uint32 as opus_uint32
    }
    ec_enc_normalize(_this);
}

pub unsafe extern "C" fn ec_enc_uint(mut _this: *mut ec_enc,
                                     mut _fl: opus_uint32,
                                     mut _ft: opus_uint32) -> () {
    let mut ft: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    let mut ftb: libc::c_int = 0;
    if !(_ft > 1i32 as libc::c_uint) {
        return celt_fatal ((*::std::mem::transmute::<&[u8; 24],
                                             &mut [libc::c_char; 24]>(b"assertion failed: _ft>1\x00")).as_mut_ptr(),
                   (*::std::mem::transmute::<&[u8; 14],
                                             &mut [libc::c_char; 14]>(b"celt/entenc.c\x00")).as_mut_ptr(),
                   180i32);
    } else {
        _ft = _ft.wrapping_sub(1);
        ftb =
            ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong as
                libc::c_int * 8i32 - _ft.leading_zeros() as i32;
        if ftb > 8i32 {
            ftb -= 8i32;
            ft = (_ft >> ftb).wrapping_add(1i32 as libc::c_uint);
            fl = _fl >> ftb;
            ec_encode(_this, fl, fl.wrapping_add(1i32 as libc::c_uint), ft);
            ec_enc_bits(_this,
                        _fl &
                            ((1i32 as opus_uint32) << ftb).wrapping_sub(1u32),
                        ftb as libc::c_uint);
        } else {
            ec_encode(_this, _fl, _fl.wrapping_add(1i32 as libc::c_uint),
                      _ft.wrapping_add(1i32 as libc::c_uint));
        }
        return;
    };
}

pub unsafe extern "C" fn ec_enc_bits(mut _this: *mut ec_enc,
                                     mut _fl: opus_uint32,
                                     mut _bits: libc::c_uint) -> () {
    let mut window: ec_window = 0;
    let mut used: libc::c_int = 0;
    window = (*_this).end_window;
    used = (*_this).nend_bits;
    if !(_bits > 0i32 as libc::c_uint) {
        return celt_fatal((*::std::mem::transmute::<&[u8; 26],
                                             &mut [libc::c_char; 26]>(b"assertion failed: _bits>0\x00")).as_mut_ptr(),
                   (*::std::mem::transmute::<&[u8; 14],
                                             &mut [libc::c_char; 14]>(b"celt/entenc.c\x00")).as_mut_ptr(),
                   198i32);
    } else {
        if (used as libc::c_uint).wrapping_add(_bits) >
               (::std::mem::size_of::<ec_window>() as libc::c_ulong as
                    libc::c_int * 8i32) as libc::c_uint {
            loop  {
                (*_this).error |=
                    ec_write_byte_at_end(_this,
                                         window &
                                             (1u32 <<
                                                  8i32).wrapping_sub(1i32 as
                                                                         libc::c_uint));
                window >>= 8i32;
                used -= 8i32;
                if !(used >= 8i32) { break ; }
            }
        }
        window |= _fl << used;
        used =
            (used as libc::c_uint).wrapping_add(_bits) as libc::c_int as
                libc::c_int;
        (*_this).end_window = window;
        (*_this).nend_bits = used;
        (*_this).nbits_total =
            ((*_this).nbits_total as libc::c_uint).wrapping_add(_bits) as
                libc::c_int as libc::c_int;
        return;
    };
}
unsafe extern "C" fn ec_write_byte_at_end(mut _this: *mut ec_enc,
                                          mut _value: libc::c_uint)
 -> libc::c_int {
    if (*_this).offs.wrapping_add((*_this).end_offs) >= (*_this).storage {
        return -1i32
    } else {
        (*_this).end_offs = (*_this).end_offs.wrapping_add(1);
        *(*_this).buf.offset((*_this).storage.wrapping_sub((*_this).end_offs)
                                 as isize) = _value as libc::c_uchar;
        return 0i32
    };
}

pub unsafe extern "C" fn ec_enc_patch_initial_bits(mut _this: *mut ec_enc,
                                                   mut _val: libc::c_uint,
                                                   mut _nbits: libc::c_uint)
 -> () {
    let mut shift: libc::c_int = 0;
    let mut mask: libc::c_uint = 0;
    if !(_nbits <= 8i32 as libc::c_uint) {
        return celt_fatal((*::std::mem::transmute::<&[u8; 38],
                                             &mut [libc::c_char; 38]>(b"assertion failed: _nbits<=EC_SYM_BITS\x00")).as_mut_ptr(),
                   (*::std::mem::transmute::<&[u8; 14],
                                             &mut [libc::c_char; 14]>(b"celt/entenc.c\x00")).as_mut_ptr(),
                   217i32);
    } else {
        shift = (8i32 as libc::c_uint).wrapping_sub(_nbits) as libc::c_int;
        mask = ((1i32 << _nbits) - 1i32 << shift) as libc::c_uint;
        if (*_this).offs > 0i32 as libc::c_uint {
            *(*_this).buf.offset(0isize) =
                (*(*_this).buf.offset(0isize) as libc::c_uint & !mask |
                     _val << shift) as libc::c_uchar
        } else if (*_this).rem >= 0i32 {
            (*_this).rem =
                ((*_this).rem as libc::c_uint & !mask | _val << shift) as
                    libc::c_int
        } else if (*_this).rng <= 1u32 << 32i32 - 1i32 >> _nbits {
            (*_this).val =
                (*_this).val & !(mask << 32i32 - 8i32 - 1i32) |
                    _val << 32i32 - 8i32 - 1i32 + shift
        } else { (*_this).error = -1i32 }
        return;
    };
}


pub unsafe extern "C" fn ec_enc_done(mut _this: *mut ec_enc) -> () {
    let mut window: ec_window = 0;
    let mut used: libc::c_int = 0;
    let mut msk: opus_uint32 = 0;
    let mut end: opus_uint32 = 0;
    let mut l: libc::c_int = 0;
    l =
        32i32 -
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong as
                 libc::c_int * 8i32 - (*_this).rng.leading_zeros() as i32);
    msk = (1u32 << 32i32 - 1i32).wrapping_sub(1i32 as libc::c_uint) >> l;
    end = (*_this).val.wrapping_add(msk) & !msk;
    if end | msk >= (*_this).val.wrapping_add((*_this).rng) {
        l += 1;
        msk >>= 1i32;
        end = (*_this).val.wrapping_add(msk) & !msk;
    };
    while l > 0i32 {
        ec_enc_carry_out(_this, (end >> 32i32 - 8i32 - 1i32) as libc::c_int);
        end =
            end << 8i32 &
                (1u32 << 32i32 - 1i32).wrapping_sub(1i32 as libc::c_uint);
        l -= 8i32;
    }
    if (*_this).rem >= 0i32 || (*_this).ext > 0i32 as libc::c_uint {
        ec_enc_carry_out(_this, 0i32);
    };
    window = (*_this).end_window;
    used = (*_this).nend_bits;
    while used >= 8i32 {
        (*_this).error |=
            ec_write_byte_at_end(_this,
                                 window &
                                     (1u32 <<
                                          8i32).wrapping_sub(1i32 as
                                                                 libc::c_uint));
        window >>= 8i32;
        used -= 8i32;
    }
    if 0 == (*_this).error {
     // FIXME: Skipping memory clearing
        if used > 0i32 {
            if (*_this).end_offs >= (*_this).storage {
                (*_this).error = -1i32;
            } else {
                l = -l;
                if (*_this).offs.wrapping_add((*_this).end_offs) >=
                       (*_this).storage && l < used {
                    window &= ((1i32 << l) - 1i32) as libc::c_uint;
                    (*_this).error = -1i32;
                };
                let ref mut fresh2 =
                    *(*_this).buf.offset((*_this).storage.wrapping_sub((*_this).end_offs).wrapping_sub(1i32
                                                                                                           as
                                                                                                           libc::c_uint)
                                             as isize);
                *fresh2 =
                    (*fresh2 as libc::c_int |
                         window as libc::c_uchar as libc::c_int) as
                        libc::c_uchar;
            };
        };
    };
}

fn celt_fatal(str: *const libc::c_char, file: *const libc::c_char,
                line: libc::c_int)
{
    panic!("celt_fatal")
}
