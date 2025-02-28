#![allow(unused)]

pub const N1: u32 = -1i32 as u32;
pub const N2: u32 = -2i32 as u32;

pub const SUM_TRIPLES: &[(&[u32], &[u32], &[u32])] = &[
    (&[], &[], &[]),
    (&[], &[1], &[1]),
    (&[1], &[1], &[2]),
    (&[1], &[1, 1], &[2, 1]),
    (&[1], &[N1], &[0, 1]),
    (&[1], &[N1, N1], &[0, 0, 1]),
    (&[N1, N1], &[N1, N1], &[N2, N1, 1]),
    (&[1, 1, 1], &[N1, N1], &[0, 1, 2]),
    (&[2, 2, 1], &[N1, N2], &[1, 1, 2]),
    (&[1, 2, 2, 1], &[N1, N2], &[0, 1, 3, 1]),
];

pub const M: u32 = u32::MAX;
pub const MUL_TRIPLES: &[(&[u32], &[u32], &[u32])] = &[
    (&[], &[], &[]),
    (&[], &[1], &[]),
    (&[2], &[], &[]),
    (&[1], &[1], &[1]),
    (&[2], &[3], &[6]),
    (&[1], &[1, 1, 1], &[1, 1, 1]),
    (&[1, 2, 3], &[3], &[3, 6, 9]),
    (&[1, 1, 1], &[N1], &[N1, N1, N1]),
    (&[1, 2, 3], &[N1], &[N1, N2, N2, 2]),
    (&[1, 2, 3, 4], &[N1], &[N1, N2, N2, N2, 3]),
    (&[N1], &[N1], &[1, N2]),
    (&[N1, N1], &[N1], &[1, N1, N2]),
    (&[N1, N1, N1], &[N1], &[1, N1, N1, N2]),
    (&[N1, N1, N1, N1], &[N1], &[1, N1, N1, N1, N2]),
    (&[M / 2 + 1], &[2], &[0, 1]),
    (&[0, M / 2 + 1], &[2], &[0, 0, 1]),
    (&[1, 2], &[1, 2, 3], &[1, 4, 7, 6]),
    (&[N1, N1], &[N1, N1, N1], &[1, 0, N1, N2, N1]),
    (&[N1, N1, N1], &[N1, N1, N1, N1], &[1, 0, 0, N1, N2, N1, N1]),
    (&[0, 0, 1], &[1, 2, 3], &[0, 0, 1, 2, 3]),
    (&[0, 0, 1], &[0, 0, 0, 1], &[0, 0, 0, 0, 0, 1]),
];

#[allow(clippy::type_complexity)]
pub const DIV_REM_QUADRUPLES: &[(&[u32], &[u32], &[u32], &[u32])] = &[
    (&[1], &[2], &[], &[1]),
    (&[3], &[2], &[1], &[1]),
    (&[1, 1], &[2], &[M / 2 + 1], &[1]),
    (&[1, 1, 1], &[2], &[M / 2 + 1, M / 2 + 1], &[1]),
    (&[0, 1], &[N1], &[1], &[1]),
    (&[N1, N1], &[N2], &[2, 1], &[3]),
];
