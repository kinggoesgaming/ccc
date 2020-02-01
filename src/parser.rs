pub mod precessing_token;
pub mod token;

macro_rules! keyword {
    ($name:ident, $keyword:expr) => {
        pub fn $name(input: &str) -> nom::IResult<&str, &str> {
            nom::bytes::streaming::tag($keyword)(input)
        }
    };
}

keyword!(parse_keyword_alignof, "alignof");
keyword!(parse_keyword_auto, "auto");
keyword!(parse_keyword_break, "break");
keyword!(parse_keyword_case, "case");
keyword!(parse_keyword_char, "char");
keyword!(parse_keyword_const, "const");
keyword!(parse_keyword_continue, "continue");
keyword!(parse_keyword_default, "default");
keyword!(parse_keyword_do, "do");
keyword!(parse_keyword_double, "double");
keyword!(parse_keyword_else, "else");
keyword!(parse_keyword_enum, "enum");
keyword!(parse_keyword_extern, "extern");
keyword!(parse_keyword_float, "float");
keyword!(parse_keyword_for, "for");
keyword!(parse_keyword_goto, "goto");
keyword!(parse_keyword_if, "if");
keyword!(parse_keyword_inline, "inline");
keyword!(parse_keyword_int, "int");
keyword!(parse_keyword_long, "long");
keyword!(parse_keyword_register, "register");
keyword!(parse_keyword_restrict, "restrict");
keyword!(parse_keyword_return, "return");
keyword!(parse_keyword_short, "short");
keyword!(parse_keyword_signed, "signed");
keyword!(parse_keyword_sizeof, "sizeof");
keyword!(parse_keyword_static, "static");
keyword!(parse_keyword_struct, "struct");
keyword!(parse_keyword_switch, "switch");
keyword!(parse_keyword_typedef, "typedef");
keyword!(parse_keyword_union, "union");
keyword!(parse_keyword_unsigned, "unsigned");
keyword!(parse_keyword_void, "void");
keyword!(parse_keyword_volatile, "volatile");
keyword!(parse_keyword_while, "while");
keyword!(parse_keyword_alignas, "__Alignas");
keyword!(parse_keyword_atomic, "__Atomic");
keyword!(parse_keyword_bool, "__Bool");
keyword!(parse_keyword_complex, "__Complex");
keyword!(parse_keyword_generic, "__Generic");
keyword!(parse_keyword_imaginary, "__Imaginary");
keyword!(parse_keyword_noreturn, "__Noreturn");
keyword!(parse_keyword_static_assert, "__Static_assert");
keyword!(parse_keyword_thread_local, "__Thread_local");
