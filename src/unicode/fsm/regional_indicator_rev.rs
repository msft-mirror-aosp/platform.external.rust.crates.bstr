// DO NOT EDIT THIS FILE. IT WAS AUTOMATICALLY GENERATED BY:
//
//   ucd-generate dfa --name REGIONAL_INDICATOR_REV --reverse --classes --minimize --anchored --premultiply --state-size 1 src/unicode/fsm/ \p{gcb=Regional_Indicator}
//
// ucd-generate 0.2.9 is available on crates.io.

#[cfg(target_endian = "big")]
lazy_static::lazy_static! {
  pub static ref REGIONAL_INDICATOR_REV: ::regex_automata::DenseDFA<&'static [u8], u8> = {
    #[repr(C)]
    struct Aligned<B: ?Sized> {
        _align: [u8; 0],
        bytes: B,
    }

    static ALIGNED: &'static Aligned<[u8]> = &Aligned {
        _align: [],
        bytes: *include_bytes!("regional_indicator_rev.bigendian.dfa"),
    };

    unsafe {
      ::regex_automata::DenseDFA::from_bytes(&ALIGNED.bytes)
    }
  };
}

#[cfg(target_endian = "little")]
lazy_static::lazy_static! {
  pub static ref REGIONAL_INDICATOR_REV: ::regex_automata::DenseDFA<&'static [u8], u8> = {
    #[repr(C)]
    struct Aligned<B: ?Sized> {
        _align: [u8; 0],
        bytes: B,
    }

    static ALIGNED: &'static Aligned<[u8]> = &Aligned {
        _align: [],
        bytes: *include_bytes!("regional_indicator_rev.littleendian.dfa"),
    };

    unsafe {
      ::regex_automata::DenseDFA::from_bytes(&ALIGNED.bytes)
    }
  };
}
