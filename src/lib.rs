pub mod cesar;

fn bezout(coeff_to_invert: u8, modulo: u8) -> u8 {
    let (mut old_r, mut r) = (modulo, coeff_to_invert);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r!=0 {
        let quot = old_r;
        (old_r, r) = (r, old_r - quot*r);
        (old_s, s) = (s, old_s - quot*s);
        (old_t, t) = (t, old_t - quot*t);
    }

    old_t
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bezout() {
        let inv = bezout(5, 23);
        assert!((5*inv%23)==1);
    }
}

