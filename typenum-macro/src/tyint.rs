use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Error as SynError, LitInt, Result as SynResult};

pub fn f_tyint(literal: LitInt) -> SynResult<TokenStream> {
    let unsigned_tokens: [TokenStream; 64] = [
        quote! { typenum::consts::U1 },
        quote! { typenum::consts::U2 },
        quote! { typenum::consts::U4 },
        quote! { typenum::consts::U8 },
        quote! { typenum::consts::U16 },
        quote! { typenum::consts::U32 },
        quote! { typenum::consts::U64 },
        quote! { typenum::consts::U128 },
        quote! { typenum::consts::U256 },
        quote! { typenum::consts::U512 },
        quote! { typenum::consts::U1024 },
        quote! { typenum::consts::U2048 },
        quote! { typenum::consts::U4096 },
        quote! { typenum::consts::U8192 },
        quote! { typenum::consts::U16384 },
        quote! { typenum::consts::U32768 },
        quote! { typenum::consts::U65536 },
        quote! { typenum::consts::U131072 },
        quote! { typenum::consts::U262144 },
        quote! { typenum::consts::U524288 },
        quote! { typenum::consts::U1048576 },
        quote! { typenum::consts::U2097152 },
        quote! { typenum::consts::U4194304 },
        quote! { typenum::consts::U8388608 },
        quote! { typenum::consts::U16777216 },
        quote! { typenum::consts::U33554432 },
        quote! { typenum::consts::U67108864 },
        quote! { typenum::consts::U134217728 },
        quote! { typenum::consts::U268435456 },
        quote! { typenum::consts::U536870912 },
        quote! { typenum::consts::U1073741824 },
        quote! { typenum::consts::U2147483648 },
        quote! { typenum::consts::U4294967296 },
        quote! { typenum::consts::U8589934592 },
        quote! { typenum::consts::U17179869184 },
        quote! { typenum::consts::U34359738368 },
        quote! { typenum::consts::U68719476736 },
        quote! { typenum::consts::U137438953472 },
        quote! { typenum::consts::U274877906944 },
        quote! { typenum::consts::U549755813888 },
        quote! { typenum::consts::U1099511627776 },
        quote! { typenum::consts::U2199023255552 },
        quote! { typenum::consts::U4398046511104 },
        quote! { typenum::consts::U8796093022208 },
        quote! { typenum::consts::U17592186044416 },
        quote! { typenum::consts::U35184372088832 },
        quote! { typenum::consts::U70368744177664 },
        quote! { typenum::consts::U140737488355328 },
        quote! { typenum::consts::U281474976710656 },
        quote! { typenum::consts::U562949953421312 },
        quote! { typenum::consts::U1125899906842624 },
        quote! { typenum::consts::U2251799813685248 },
        quote! { typenum::consts::U4503599627370496 },
        quote! { typenum::consts::U9007199254740992 },
        quote! { typenum::consts::U18014398509481984 },
        quote! { typenum::consts::U36028797018963968 },
        quote! { typenum::consts::U72057594037927936 },
        quote! { typenum::consts::U144115188075855872 },
        quote! { typenum::consts::U288230376151711744 },
        quote! { typenum::consts::U576460752303423488 },
        quote! { typenum::consts::U1152921504606846976 },
        quote! { typenum::consts::U2305843009213693952 },
        quote! { typenum::consts::U4611686018427387904 },
        quote! { typenum::consts::U9223372036854775808 },
    ];

    let positive_tokens: [TokenStream; 63] = [
        quote! { typenum::consts::P1 },
        quote! { typenum::consts::P2 },
        quote! { typenum::consts::P4 },
        quote! { typenum::consts::P8 },
        quote! { typenum::consts::P16 },
        quote! { typenum::consts::P32 },
        quote! { typenum::consts::P64 },
        quote! { typenum::consts::P128 },
        quote! { typenum::consts::P256 },
        quote! { typenum::consts::P512 },
        quote! { typenum::consts::P1024 },
        quote! { typenum::consts::P2048 },
        quote! { typenum::consts::P4096 },
        quote! { typenum::consts::P8192 },
        quote! { typenum::consts::P16384 },
        quote! { typenum::consts::P32768 },
        quote! { typenum::consts::P65536 },
        quote! { typenum::consts::P131072 },
        quote! { typenum::consts::P262144 },
        quote! { typenum::consts::P524288 },
        quote! { typenum::consts::P1048576 },
        quote! { typenum::consts::P2097152 },
        quote! { typenum::consts::P4194304 },
        quote! { typenum::consts::P8388608 },
        quote! { typenum::consts::P16777216 },
        quote! { typenum::consts::P33554432 },
        quote! { typenum::consts::P67108864 },
        quote! { typenum::consts::P134217728 },
        quote! { typenum::consts::P268435456 },
        quote! { typenum::consts::P536870912 },
        quote! { typenum::consts::P1073741824 },
        quote! { typenum::consts::P2147483648 },
        quote! { typenum::consts::P4294967296 },
        quote! { typenum::consts::P8589934592 },
        quote! { typenum::consts::P17179869184 },
        quote! { typenum::consts::P34359738368 },
        quote! { typenum::consts::P68719476736 },
        quote! { typenum::consts::P137438953472 },
        quote! { typenum::consts::P274877906944 },
        quote! { typenum::consts::P549755813888 },
        quote! { typenum::consts::P1099511627776 },
        quote! { typenum::consts::P2199023255552 },
        quote! { typenum::consts::P4398046511104 },
        quote! { typenum::consts::P8796093022208 },
        quote! { typenum::consts::P17592186044416 },
        quote! { typenum::consts::P35184372088832 },
        quote! { typenum::consts::P70368744177664 },
        quote! { typenum::consts::P140737488355328 },
        quote! { typenum::consts::P281474976710656 },
        quote! { typenum::consts::P562949953421312 },
        quote! { typenum::consts::P1125899906842624 },
        quote! { typenum::consts::P2251799813685248 },
        quote! { typenum::consts::P4503599627370496 },
        quote! { typenum::consts::P9007199254740992 },
        quote! { typenum::consts::P18014398509481984 },
        quote! { typenum::consts::P36028797018963968 },
        quote! { typenum::consts::P72057594037927936 },
        quote! { typenum::consts::P144115188075855872 },
        quote! { typenum::consts::P288230376151711744 },
        quote! { typenum::consts::P576460752303423488 },
        quote! { typenum::consts::P1152921504606846976 },
        quote! { typenum::consts::P2305843009213693952 },
        quote! { typenum::consts::P4611686018427387904 },
    ];

    let negative_tokens: [TokenStream; 63] = [
        quote! { typenum::consts::N1 },
        quote! { typenum::consts::N2 },
        quote! { typenum::consts::N4 },
        quote! { typenum::consts::N8 },
        quote! { typenum::consts::N16 },
        quote! { typenum::consts::N32 },
        quote! { typenum::consts::N64 },
        quote! { typenum::consts::N128 },
        quote! { typenum::consts::N256 },
        quote! { typenum::consts::N512 },
        quote! { typenum::consts::N1024 },
        quote! { typenum::consts::N2048 },
        quote! { typenum::consts::N4096 },
        quote! { typenum::consts::N8192 },
        quote! { typenum::consts::N16384 },
        quote! { typenum::consts::N32768 },
        quote! { typenum::consts::N65536 },
        quote! { typenum::consts::N131072 },
        quote! { typenum::consts::N262144 },
        quote! { typenum::consts::N524288 },
        quote! { typenum::consts::N1048576 },
        quote! { typenum::consts::N2097152 },
        quote! { typenum::consts::N4194304 },
        quote! { typenum::consts::N8388608 },
        quote! { typenum::consts::N16777216 },
        quote! { typenum::consts::N33554432 },
        quote! { typenum::consts::N67108864 },
        quote! { typenum::consts::N134217728 },
        quote! { typenum::consts::N268435456 },
        quote! { typenum::consts::N536870912 },
        quote! { typenum::consts::N1073741824 },
        quote! { typenum::consts::N2147483648 },
        quote! { typenum::consts::N4294967296 },
        quote! { typenum::consts::N8589934592 },
        quote! { typenum::consts::N17179869184 },
        quote! { typenum::consts::N34359738368 },
        quote! { typenum::consts::N68719476736 },
        quote! { typenum::consts::N137438953472 },
        quote! { typenum::consts::N274877906944 },
        quote! { typenum::consts::N549755813888 },
        quote! { typenum::consts::N1099511627776 },
        quote! { typenum::consts::N2199023255552 },
        quote! { typenum::consts::N4398046511104 },
        quote! { typenum::consts::N8796093022208 },
        quote! { typenum::consts::N17592186044416 },
        quote! { typenum::consts::N35184372088832 },
        quote! { typenum::consts::N70368744177664 },
        quote! { typenum::consts::N140737488355328 },
        quote! { typenum::consts::N281474976710656 },
        quote! { typenum::consts::N562949953421312 },
        quote! { typenum::consts::N1125899906842624 },
        quote! { typenum::consts::N2251799813685248 },
        quote! { typenum::consts::N4503599627370496 },
        quote! { typenum::consts::N9007199254740992 },
        quote! { typenum::consts::N18014398509481984 },
        quote! { typenum::consts::N36028797018963968 },
        quote! { typenum::consts::N72057594037927936 },
        quote! { typenum::consts::N144115188075855872 },
        quote! { typenum::consts::N288230376151711744 },
        quote! { typenum::consts::N576460752303423488 },
        quote! { typenum::consts::N1152921504606846976 },
        quote! { typenum::consts::N2305843009213693952 },
        quote! { typenum::consts::N4611686018427387904 },
    ];

    let tokens = match literal.suffix() {
        "u8" | "u16" | "u32" | "u64" | "usize" => {
            let value = literal.base10_parse::<u64>()?;

            let (tokens, _) = unsigned_tokens.into_iter().fold(
                (quote! { typenum::consts::U0 }, value),
                |(tokens, remainder), tk| {
                    let new_tokens = if (remainder & 1) != 0 {
                        quote! {  typenum::Sum<#tk, #tokens> }
                    } else {
                        tokens
                    };

                    (new_tokens, remainder >> 1)
                },
            );

            tokens
        }
        "" | "i8" | "i16" | "i32" | "i64" | "isize" => {
            let value = literal.base10_parse::<i128>()?;

            if value >= 0 {
                let (tokens, _) = positive_tokens.into_iter().fold(
                    (quote! { typenum::consts::Z0 }, value),
                    |(tokens, remainder), tk| {
                        let new_tokens = if (remainder & 1) != 0 {
                            quote! {  typenum::Sum<#tk, #tokens> }
                        } else {
                            tokens
                        };

                        (new_tokens, remainder >> 1)
                    },
                );
                tokens
            } else {
                let (tokens, _) = negative_tokens.into_iter().fold(
                    (quote! { typenum::consts::Z0 }, -value),
                    |(tokens, remainder), tk| {
                        let new_tokens = if (remainder & 1) != 0 {
                            quote! {  typenum::Sum<#tk, #tokens> }
                        } else {
                            tokens
                        };

                        (new_tokens, remainder >> 1)
                    },
                );
                tokens
            }
        }
        _ => {
            let error = SynError::new(
                literal.span(),
                format!("Suffix {:?} is not supported", literal.suffix()),
            );
            return Err(error);
        }
    };

    Ok(tokens)
}
