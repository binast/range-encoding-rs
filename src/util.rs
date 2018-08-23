// Copyright (c) 2017-2018, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

/// Fixed point arithmetics.
pub trait Fixed {
  fn floor_log2(&self, n: usize) -> usize;
  fn ceil_log2(&self, n: usize) -> usize;
  fn align_power_of_two(&self, n: usize) -> usize;
  fn align_power_of_two_and_shift(&self, n: usize) -> usize;
}

impl Fixed for usize {
  #[inline]
  fn floor_log2(&self, n: usize) -> usize {
    self & !((1 << n) - 1)
  }
  #[inline]
  fn ceil_log2(&self, n: usize) -> usize {
    (self + (1 << n) - 1).floor_log2(n)
  }
  #[inline]
  fn align_power_of_two(&self, n: usize) -> usize {
    self.ceil_log2(n)
  }
  #[inline]
  fn align_power_of_two_and_shift(&self, n: usize) -> usize {
    (self + (1 << n) - 1) >> n
  }
}
use num_traits::PrimInt;
use std::mem::size_of;

/// Fast `log` for primitive integers.
pub trait ILog : PrimInt {
    fn ilog(self) -> Self {
        Self::from(size_of::<Self>() * 8 - self.leading_zeros() as usize).unwrap()
    }
}

impl<T> ILog for T where T: PrimInt {}
