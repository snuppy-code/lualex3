use std::fmt::{self, Display};

#[derive(Debug, PartialEq)]
pub enum LexerError {
    UnclosedLongComment,

    InvalidStringuEscapeMissingOpenBrace,
    InvalidStringuEscapeMissingClosingBrace,
    InvalidStringuEscapeInvalidChar(u8),
    InvalidStringuEscapeNoDigits,
    InvalidStringuEscapeBeyond0x10FFFF,
    
    InvalidStringxEscapeUnfinished,
    InvalidStringxEscapeInvalidPossiblyTooLarge,

    InvalidStringDecimalEscapeBeyondu8,
    InvalidStringEscape,
    UnclosedShortString,
    UnclosedLongString,
    DecimalNumericConstantNeedsIntOrFracPart,
    HexNumericConstantNeedsIntOrFracPart,
    HexNumericConstantMalformedExponent,
    DecimalNumericConstantMalformedExponent,
}

impl<'i> Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Self::FailedLexingAll(s) => write!(f,"Encountered unexpected input! remains: `{}`",s),
            Self::UnclosedLongComment => write!(f, "Unclosed long comment !"),
            
            Self::InvalidStringuEscapeMissingOpenBrace => write!(f, "Invalid \\u string escape: Missing opening brace !"),
            Self::InvalidStringuEscapeMissingClosingBrace => write!(f, "Invalid \\u string escape: Missing closing brace !"),
            Self::InvalidStringuEscapeInvalidChar(b) => write!(f, "Invalid \\u string escape: Invalid character (byte value: {b}) in escape sequence !"),
            Self::InvalidStringuEscapeNoDigits => write!(f, "Invalid \\u string escape: No digits in escape sequence ! (needed to specify codepoint)"),
            Self::InvalidStringuEscapeBeyond0x10FFFF => write!(f, "Invalid \\u string escape: Beyond Lua 5.3 0x10FFFF limit !"),

            Self::InvalidStringxEscapeUnfinished => write!(f, "Unfinished \\x escape sequence at end of string !"),
            Self::InvalidStringxEscapeInvalidPossiblyTooLarge => write!(f, "Invalid hex, maybe too large? in \\x escape sequence !"),

            Self::DecimalNumericConstantNeedsIntOrFracPart => write!(f, "Decimal numeric constant needs at least an integer or fractional part !"),
            Self::HexNumericConstantNeedsIntOrFracPart => write!(f, "Hex numeric constant needs integer part or fractional part!"),
            Self::HexNumericConstantMalformedExponent => write!(f, "Hex numeric constant has malformed exponent"),
            Self::DecimalNumericConstantMalformedExponent => write!(f, "Decimal numeric constant has malformed exponent"),

            Self::InvalidStringDecimalEscapeBeyondu8 => write!(f, "Invalid decimal string escape: Beyond 255 !"),
            Self::InvalidStringEscape => write!(f, "Unrecognized string escape !"),
            Self::UnclosedShortString => write!(f, "Unclosed short string !"),
            Self::UnclosedLongString => write!(f, "Unclosed long string !"),
        }
	}
}