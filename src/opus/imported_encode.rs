//! This file was generated by c2rust, then heavily modified by hand.

#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]

use std;
use std::io::Write;

pub type int32_t = std::os::raw::c_int;
pub type uint32_t = std::os::raw::c_uint;
pub type __uint16_t = std::os::raw::c_ushort;
pub type __uint32_t = std::os::raw::c_uint;
pub type __int64_t = std::os::raw::c_longlong;
pub type __uint64_t = std::os::raw::c_ulonglong;
pub type __darwin_size_t = std::os::raw::c_ulong;
pub type __darwin_off_t = __int64_t;
pub type opus_int32 = int32_t;
pub type opus_uint32 = uint32_t;
pub type size_t = __darwin_size_t;

pub struct ec_enc<W> where W: std::io::Write {
    pub out: W,
    pub end_window: ec_window,
    pub nend_bits: std::os::raw::c_int,
    pub nbits_total: std::os::raw::c_int,
    pub offs: opus_uint32,
    pub rng: opus_uint32,
    pub val: opus_uint32,
    pub ext: opus_uint32,
    pub rem: std::os::raw::c_int,

    /// Bytes written at the end, in backwards order.
    pub end_buffer: Vec<u8>,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed {
    __f: std::os::raw::c_double,
    __u: std::os::raw::c_ulonglong,
}

#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_0 {
    pub __m: std::os::raw::c_ulonglong,
    pub __sexp: std::os::raw::c_ushort,
}
pub type ec_window = opus_uint32;
pub type fpos_t = __darwin_off_t;

fn celt_udiv(mut n: opus_uint32, mut d: opus_uint32) -> opus_uint32 {
    return n.wrapping_div(d);
}

pub unsafe fn ec_encode<W: Write>(mut _this: *mut ec_enc<W>,
                                   mut _fl: std::os::raw::c_uint,
                                   mut _fh: std::os::raw::c_uint,
                                   mut _ft: std::os::raw::c_uint) -> Result<(), std::io::Error> {
    let mut r = celt_udiv((*_this).rng, _ft);
    if _fl > 0i32 as std::os::raw::c_uint {
        (*_this).val =
            ((*_this).val as
                 std::os::raw::c_uint).wrapping_add((*_this).rng.wrapping_sub(r.wrapping_mul(_ft.wrapping_sub(_fl))))
                as opus_uint32 as opus_uint32;
        (*_this).rng = r.wrapping_mul(_fh.wrapping_sub(_fl))
    } else {
        (*_this).rng =
            ((*_this).rng as
                 std::os::raw::c_uint).wrapping_sub(r.wrapping_mul(_ft.wrapping_sub(_fh)))
                as opus_uint32 as opus_uint32
    }
    ec_enc_normalize(_this)?;
    Ok(())
}
unsafe fn ec_enc_normalize<W: Write>(mut _this: *mut ec_enc<W>) -> Result<(), std::io::Error> {
    while (*_this).rng <= 1u32 << 32i32 - 1i32 >> 8i32 {
        ec_enc_carry_out(_this,
                         ((*_this).val >> 32i32 - 8i32 - 1i32) as
                             std::os::raw::c_int)?;
        (*_this).val =
            (*_this).val << 8i32 &
                (1u32 << 32i32 - 1i32).wrapping_sub(1i32 as std::os::raw::c_uint);
        (*_this).rng <<= 8i32;
        (*_this).nbits_total += 8i32
    };
    Ok(())
}
unsafe fn ec_enc_carry_out<W: Write>(mut _this: *mut ec_enc<W>,
                                      mut _c: std::os::raw::c_int) -> Result<(), std::io::Error> {
    if _c as std::os::raw::c_uint != (1u32 << 8i32).wrapping_sub(1i32 as std::os::raw::c_uint)
       {
        let mut carry = _c >> 8i32;
        if (*_this).rem >= 0i32 {
            ec_write_byte(_this, ((*_this).rem + carry) as std::os::raw::c_uint)?;
        }
        if (*_this).ext > 0i32 as std::os::raw::c_uint {
            let mut sym =
                (1u32 <<
                     8i32).wrapping_sub(1i32 as
                                            std::os::raw::c_uint).wrapping_add(carry
                                                                           as
                                                                           std::os::raw::c_uint)
                    & (1u32 << 8i32).wrapping_sub(1i32 as std::os::raw::c_uint);
            loop  {
                ec_write_byte(_this, sym)?;
                (*_this).ext = (*_this).ext.wrapping_sub(1);
                if !((*_this).ext > 0i32 as std::os::raw::c_uint) { break ; }
            }
        }
        (*_this).rem =
            (_c as std::os::raw::c_uint &
                 (1u32 << 8i32).wrapping_sub(1i32 as std::os::raw::c_uint)) as
                std::os::raw::c_int
    } else { (*_this).ext = (*_this).ext.wrapping_add(1) };
    Ok(())
}
unsafe fn ec_write_byte<W: Write>(mut _this: *mut ec_enc<W>,
                           mut _value: std::os::raw::c_uint) -> Result<(), std::io::Error> {
    (*_this).out.write_all(&[_value as u8])?;
    Ok(())
}

pub unsafe fn ec_encode_bin<W: Write>(mut _this: *mut ec_enc<W>,
                                       mut _fl: std::os::raw::c_uint,
                                       mut _fh: std::os::raw::c_uint,
                                       mut _bits: std::os::raw::c_uint) -> Result<(), std::io::Error> {
    let mut r = (*_this).rng >> _bits;
    if _fl > 0i32 as std::os::raw::c_uint {
        (*_this).val =
            ((*_this).val as
                 std::os::raw::c_uint).wrapping_add((*_this).rng.wrapping_sub(r.wrapping_mul((1u32
                                                                                          <<
                                                                                          _bits).wrapping_sub(_fl))))
                as opus_uint32 as opus_uint32;
        (*_this).rng = r.wrapping_mul(_fh.wrapping_sub(_fl))
    } else {
        (*_this).rng =
            ((*_this).rng as
                 std::os::raw::c_uint).wrapping_sub(r.wrapping_mul((1u32 <<
                                                                _bits).wrapping_sub(_fh)))
                as opus_uint32 as opus_uint32
    }
    ec_enc_normalize(_this)?;
    Ok(())
}

pub unsafe fn ec_enc_bit_logp<W: Write>(mut _this: *mut ec_enc<W>,
                                         mut _val: std::os::raw::c_int,
                                         mut _logp: std::os::raw::c_uint) -> Result<(), std::io::Error> {
    let mut r = (*_this).rng;
    let mut l = (*_this).val;
    let mut s = r >> _logp;
    r = (r as std::os::raw::c_uint).wrapping_sub(s) as opus_uint32 as opus_uint32;
    if 0 != _val { (*_this).val = l.wrapping_add(r) }
    (*_this).rng = if 0 != _val { s } else { r };
    ec_enc_normalize(_this)?;
    Ok(())
}

pub unsafe fn ec_enc_icdf<W: Write>(mut _this: *mut ec_enc<W>,
                                     mut _s: std::os::raw::c_int,
                                     mut _icdf: *const std::os::raw::c_uchar,
                                     mut _ftb: std::os::raw::c_uint) -> Result<(), std::io::Error> {
    let mut r = (*_this).rng >> _ftb;
    if _s > 0i32 {
        (*_this).val =
            ((*_this).val as
                 std::os::raw::c_uint).wrapping_add((*_this).rng.wrapping_sub(r.wrapping_mul(*_icdf.offset((_s
                                                                                                        -
                                                                                                        1i32)
                                                                                                       as
                                                                                                       isize)
                                                                                         as
                                                                                         std::os::raw::c_uint)))
                as opus_uint32 as opus_uint32;
        (*_this).rng =
            r.wrapping_mul((*_icdf.offset((_s - 1i32) as isize) as std::os::raw::c_int
                                - *_icdf.offset(_s as isize) as std::os::raw::c_int)
                               as std::os::raw::c_uint)
    } else {
        (*_this).rng =
            ((*_this).rng as
                 std::os::raw::c_uint).wrapping_sub(r.wrapping_mul(*_icdf.offset(_s as
                                                                             isize)
                                                               as
                                                               std::os::raw::c_uint))
                as opus_uint32 as opus_uint32
    }
    ec_enc_normalize(_this)?;
    Ok(())
}

pub unsafe fn ec_enc_uint<W: Write>(mut _this: *mut ec_enc<W>,
                                    mut _fl: opus_uint32,
                                    mut _ft: opus_uint32) -> Result<(), std::io::Error> {
    assert!(_ft > 1);
    _ft = _ft.wrapping_sub(1);
    let mut ftb =
        ::std::mem::size_of::<std::os::raw::c_uint>() as std::os::raw::c_ulong as
            std::os::raw::c_int * 8i32 - _ft.leading_zeros() as i32;
    if ftb > 8i32 {
        ftb -= 8i32;
        let ft = (_ft >> ftb).wrapping_add(1i32 as std::os::raw::c_uint);
        let fl = _fl >> ftb;
        ec_encode(_this, fl, fl.wrapping_add(1i32 as std::os::raw::c_uint), ft)?;
        ec_enc_bits(_this,
                    _fl &
                        ((1i32 as opus_uint32) << ftb).wrapping_sub(1u32),
                    ftb as std::os::raw::c_uint)?;
    } else {
        ec_encode(_this, _fl, _fl.wrapping_add(1i32 as std::os::raw::c_uint),
                    _ft.wrapping_add(1i32 as std::os::raw::c_uint))?;
    }
    return Ok(());
}

pub unsafe fn ec_enc_bits<W: Write>(mut _this: *mut ec_enc<W>,
                                     mut _fl: opus_uint32,
                                     mut _bits: std::os::raw::c_uint) -> Result<(), std::io::Error> {
    let mut window = (*_this).end_window;
    let mut used = (*_this).nend_bits;
    assert!(_bits > 0);
    if (used as std::os::raw::c_uint).wrapping_add(_bits) >
            (::std::mem::size_of::<ec_window>() as std::os::raw::c_ulong as
                std::os::raw::c_int * 8i32) as std::os::raw::c_uint {
        loop  {
                ec_write_byte_at_end(_this,
                                        window &
                                            (1u32 <<
                                                8i32).wrapping_sub(1i32 as
                                                                        std::os::raw::c_uint))?;
            window >>= 8i32;
            used -= 8i32;
            if !(used >= 8i32) { break ; }
        }
    }
    window |= _fl << used;
    used =
        (used as std::os::raw::c_uint).wrapping_add(_bits) as std::os::raw::c_int as
            std::os::raw::c_int;
    (*_this).end_window = window;
    (*_this).nend_bits = used;
    (*_this).nbits_total =
        ((*_this).nbits_total as std::os::raw::c_uint).wrapping_add(_bits) as
            std::os::raw::c_int as std::os::raw::c_int;
    return Ok(());
}
unsafe fn ec_write_byte_at_end<W: Write>(mut _this: *mut ec_enc<W>,
                                          mut _value: std::os::raw::c_uint)
 -> Result<(), std::io::Error> {
     (*_this).end_buffer.push(_value as u8);
     Ok(())
}

pub unsafe fn ec_enc_done<W: Write>(mut _this: *mut ec_enc<W>) -> Result<(), std::io::Error> {
    let mut l =
        32i32 -
            (::std::mem::size_of::<std::os::raw::c_uint>() as std::os::raw::c_ulong as
                 std::os::raw::c_int * 8i32 - (*_this).rng.leading_zeros() as i32);
    let mut msk = (1u32 << 32i32 - 1i32).wrapping_sub(1i32 as std::os::raw::c_uint) >> l;
    let mut end = (*_this).val.wrapping_add(msk) & !msk;
    if end | msk >= (*_this).val.wrapping_add((*_this).rng) {
        l += 1;
        msk >>= 1i32;
        end = (*_this).val.wrapping_add(msk) & !msk;
    };
    while l > 0i32 {
        ec_enc_carry_out(_this, (end >> 32i32 - 8i32 - 1i32) as std::os::raw::c_int)?;
        end =
            end << 8i32 &
                (1u32 << 32i32 - 1i32).wrapping_sub(1i32 as std::os::raw::c_uint);
        l -= 8i32;
    }
    if (*_this).rem >= 0i32 || (*_this).ext > 0i32 as std::os::raw::c_uint {
        ec_enc_carry_out(_this, 0i32)?;
    };
    let mut window = (*_this).end_window;
    let mut used = (*_this).nend_bits;
    while used >= 8i32 {
        ec_write_byte_at_end(_this,
                                window &
                                    (1u32 <<
                                        8i32).wrapping_sub(1i32 as
                                                                std::os::raw::c_uint))?;
        window >>= 8i32;
        used -= 8i32;
    }

    if used > 0i32 {
        *(*_this).end_buffer.last_mut()
            .unwrap() |= window as u8;
    };

    for byte in (*_this).end_buffer.iter().rev() {
        ec_write_byte(_this, *byte as u32)?;
    }
    Ok(())
}
