#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]

use std;
use std::io::Read;

extern crate libc;
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
pub type ec_window = opus_uint32;

pub struct ec_dec<R> where R: std::io::Read {
    pub inp: R,
    pub end_window: ec_window,
    pub nend_bits: libc::c_int,
    pub nbits_total: libc::c_int,
    pub rng: opus_uint32,
    pub val: opus_uint32,
    pub ext: opus_uint32,
    pub rem: libc::c_int,
}
unsafe extern "C" fn celt_udiv(mut n: opus_uint32, mut d: opus_uint32)
 -> opus_uint32 {
    return n.wrapping_div(d);
}
unsafe extern "C" fn celt_sudiv(mut n: opus_int32, mut d: opus_int32)
 -> opus_int32 {
    return n / d;
}

pub unsafe extern "C" fn ec_dec_init<R: Read>(mut _this: *mut ec_dec<R>) -> Result<(), std::io::Error> {
    (*_this).end_window = 0i32 as ec_window;
    (*_this).nend_bits = 0i32;
    (*_this).nbits_total =
        32i32 + 1i32 - (32i32 - ((32i32 - 2i32) % 8i32 + 1i32)) / 8i32 * 8i32;
    (*_this).rng = 1u32 << (32i32 - 2i32) % 8i32 + 1i32;
    (*_this).rem = ec_read_byte(_this)? as i32;
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
    ec_dec_normalize(_this)?;
    Ok(())
}
unsafe extern "C" fn ec_dec_normalize<R: Read>(mut _this: *mut ec_dec<R>) -> Result<(), std::io::Error> {
    while (*_this).rng <= 1u32 << 32i32 - 1i32 >> 8i32 {
        let mut sym: libc::c_int = 0;
        (*_this).nbits_total += 8i32;
        (*_this).rng <<= 8i32;
        sym = (*_this).rem;
        (*_this).rem = ec_read_byte(_this)? as i32;
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
    Ok(())
}
unsafe extern "C" fn ec_read_byte<R: Read>(mut _this: *mut ec_dec<R>) -> Result<u8, std::io::Error> {
    let mut buf = [0];
    if let Err(err) = (*_this).inp.read_exact(&mut buf) {
        if let std::io::ErrorKind::UnexpectedEof = err.kind() {
            // Reading past the end returns 0.
            Ok(0)
        } else {
            Err(err)
        }
    } else {
        Ok(buf[0])
    }
}

pub unsafe extern "C" fn ec_decode<R: Read>(mut _this: *mut ec_dec<R>,
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

pub unsafe extern "C" fn ec_decode_bin<R: Read>(mut _this: *mut ec_dec<R>,
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

pub unsafe extern "C" fn ec_dec_update<R: Read>(mut _this: *mut ec_dec<R>,
                                       mut _fl: libc::c_uint,
                                       mut _fh: libc::c_uint,
                                       mut _ft: libc::c_uint) -> Result<(), std::io::Error> {
    let mut s: opus_uint32 = 0;
    s = (*_this).ext.wrapping_mul(_ft.wrapping_sub(_fh));
    (*_this).val =
        ((*_this).val as libc::c_uint).wrapping_sub(s) as opus_uint32 as
            opus_uint32;
    (*_this).rng =
        if _fl > 0i32 as libc::c_uint {
            (*_this).ext.wrapping_mul(_fh.wrapping_sub(_fl))
        } else { (*_this).rng.wrapping_sub(s) };
    ec_dec_normalize(_this)?;
    Ok(())
}

pub unsafe extern "C" fn ec_dec_bit_logp<R: Read>(mut _this: *mut ec_dec<R>,
                                         mut _logp: libc::c_uint)
 -> Result<i32, std::io::Error>
{
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
    ec_dec_normalize(_this)?;
    Ok(ret)
}

pub unsafe extern "C" fn ec_dec_icdf<R: Read>(mut _this: *mut ec_dec<R>,
                                     mut _icdf: *const libc::c_uchar,
                                     mut _ftb: libc::c_uint) -> Result<i32, std::io::Error> {
    let mut t;
    let mut s = (*_this).rng;
    let mut d = (*_this).val;
    let mut r = s >> _ftb;
    let mut ret = -1i32;
    loop  {
        t = s;
        ret += 1;
        s = r.wrapping_mul(*_icdf.offset(ret as isize) as libc::c_uint);
        if !(d < s) { break ; }
    }
    (*_this).val = d.wrapping_sub(s);
    (*_this).rng = t.wrapping_sub(s);
    ec_dec_normalize(_this)?;
    Ok(ret)
}

/*
// ec_dec_unit and ec_dec_bits need the data to be packetized,
// which is not something we do at least for the time being.

pub unsafe extern "C" fn ec_dec_uint<R: Read>(mut _this: *mut ec_dec<R>,
                                     mut _ft: opus_uint32) -> Result<u32, ()> {
    let mut ft: libc::c_uint = 0;
    let mut s: libc::c_uint = 0;
    let mut ftb: libc::c_int = 0;
    assert!(_ft > 1);
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
            Ok(t)
        } else {
            Err(())
        }
    } else {
        _ft = _ft.wrapping_add(1);
        s = ec_decode(_this, _ft);
        ec_dec_update(_this, s, s.wrapping_add(1i32 as libc::c_uint),
                        _ft);
        Ok(s)
    }
}

pub unsafe extern "C" fn ec_dec_bits<R: Read>(mut _this: *mut ec_dec<R>,
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
unsafe extern "C" fn ec_read_byte_from_end<R: Read>(mut _this: *mut ec_dec<R>)
 -> libc::c_int {
    return if (*_this).end_offs < (*_this).storage {
               (*_this).end_offs = (*_this).end_offs.wrapping_add(1);
               *(*_this).buf.offset((*_this).storage.wrapping_sub((*_this).end_offs)
                                        as isize) as libc::c_int
           } else { 0i32 };
}

*/