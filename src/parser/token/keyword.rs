macro_rules! keyword {
    ($fn:ident, $keyword:literal, $doc:literal) => {
        #[doc=$doc]
        pub fn $fn(input: &str) -> nom::IResult<&str, &str> {
            nom::bytes::streaming::tag($keyword)(input)
        }
    };
}

keyword!(
    parse_keyword_alignof,
    "alignof",
    "`alignof`"
);

keyword!(
    parse_keyword_auto,
    "auto",
    "`auto`"
);

keyword!(
    parse_keyword_break,
    "break",
    "`break`"
);

keyword!(
    parse_keyword_case,
    "case",
    "`case`"
);

keyword!(
    parse_keyword_char,
    "char",
    "`char`"
);

keyword!(
    parse_keyword_const,
    "const",
    "`const`"
);

keyword!(
    parse_keyword_continue,
    "continue",
    "`continue`"
);

keyword!(
    parse_keyword_default,
    "default",
    "`default`"
);

keyword!(
    parse_keyword_do,
    "do",
    "`do`"
);

keyword!(
    parse_keyword_double,
    "double",
    "`double`"
);

keyword!(
    parse_keyword_else,
    "else",
    "`else`"
);

keyword!(
    parse_keyword_enum,
    "enum",
    "`enum`"
);

keyword!(
    parse_keyword_extern,
    "extern",
    "`extern`"
);

keyword!(
    parse_keyword_float,
    "float",
    "`float`"
);

keyword!(
    parse_keyword_for,
    "for",
    "`for`"
);

keyword!(
    parse_keyword_goto,
    "goto",
    "`goto`"
);

keyword!(
    parse_keyword_if,
    "if",
    "`if`"
);

keyword!(
    parse_keyword_inline,
    "inline",
    "`inline`"
);

keyword!(
    parse_keyword_int,
    "int",
    "`int`"
);

keyword!(
    parse_keyword_long,
    "long",
    "`long`"
);

keyword!(
    parse_keyword_register,
    "register",
    "`register`"
);

keyword!(
    parse_keyword_restrict,
    "restrict",
    "`restrict`"
);

keyword!(
    parse_keyword_return,
    "return",
    "`return`"
);

keyword!(
    parse_keyword_short,
    "short",
    "`short`"
);

keyword!(
    parse_keyword_signed,
    "signed",
    "`signed`"
);

keyword!(
    parse_keyword_sizeof,
    "sizeof",
    "`sizeof`"
);

keyword!(
    parse_keyword_static,
    "static",
    "`static`"
);

keyword!(
    parse_keyword_struct,
    "struct",
    "`struct`"
);

keyword!(
    parse_keyword_switch,
    "switch",
    "`switch`"
);

keyword!(
    parse_keyword_typedef,
    "typedef",
    "`typedef`"
);

keyword!(
    parse_keyword_union,
    "union",
    "`union`"
);

keyword!(
    parse_keyword_unsigned,
    "unsigned",
    "`unsigned`"
);

keyword!(
    parse_keyword_void,
    "void",
    "`void`"
);

keyword!(
    parse_keyword_volatile,
    "volatile",
    "`volatile`"
);

keyword!(
    parse_keyword_while,
    "while",
    "`while`"
);

keyword!(
    parse_keyword_alignas,
    "_Alignas",
    "`_Alignas`"
);

keyword!(
    parse_keyword_atomic,
    "_Atomic",
    "`_Atomic`"
);

keyword!(
    parse_keyword_bool,
    "_Bool",
    "`_Bool`"
);

keyword!(
    parse_keyword_complex,
    "_Complex",
    "`_Complex`"
);

keyword!(
    parse_keyword_generic,
    "_Generic",
    "`_Generic`"
);

keyword!(
    parse_keyword_imaginary,
    "_Imaginary",
    "`Imaginary`"
);

keyword!(
    parse_keyword_noreturn,
    "_Noreturn",
    "`_Noreturn`"
);

keyword!(
    parse_keyword_static_assert,
    "_Static_assert",
    "`_Static_assert`"
);

keyword!(
    parse_keyword_thread_local,
    "_Thread_local",
    "`_Thread_local`"
);
