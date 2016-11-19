#![allow(dead_code)]
/// Intermediate Nodes
///
/// Mathlist    - A list of atoms
///
/// Nodes in our parse tree:
///
/// Rendering Requirements for Parsing Nodes
///
/// - Symbols
///      + Font Family & Code Point
///      + Font Dimensions [ width, height, depth ]
///      + Lig/Kern -- todo
///
/// - Operators
///      + Symbol (minus atom type)
///      + limits [true/false]
///      + Extensible?
///      + Successor?    // 
///      + largeop [true/false]?
///
/// - Delimiters
///     + Symbol (minus atom type)
///     + Extensible?
///     + Successor?
///
/// - Accent
///     + Accent Character
///     + Inner Contents:
///        - Symbol or MathList
///     + Kerning if Symbol
///
/// - Radical
///     + Inner Contents: Mathlist
///     + Superscript: Mathlist
///
/// - Scripts Environment
///     + Type: [ Atom type inherited?, if so expose? ]
///     + Subscript: Mathlist
///     + Superscript: Mathlist
///     + Base: Mathlist
///  
/// - Generalized Fraction
///     + Numerator: Mathlist
///     + Denominator: Mathlist
///     + Barwidth: TexDimension
///     + Left Delimiter: Delimiter
///     + Right Delimiter: Delimiter
///
/// - Style Changes
///     + Enum: Text/Script/ScriptScript/Display..
///
/// - Size Change
///     + Enum: Small, Large, LARGE, etc..
///
/// TODO: Environment ???

// use std::boxed::Box;

// pub type MathList = Vec<ParseNode>;
// pub type BoxedMathList = Box<MathList>;

// There are additional classes defined from unicode-math 
// in addition to those defined by TeX.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AtomType {
    Punctuation,
    Ordinal,
    Open,
    Close,
    Binary,
    Relation,
    Accent,
    AccentWide,
    BotAccent,
    BotAccentWide,
    Alpha,
    Fence,
    Operator(bool),     // bool := limits or nolimits?
    Over,
    Under,
    Inner,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Scripts {
    pub base: Option<Box<ParseNode>>,
    pub superscript: Option<Box<ParseNode>>,
    pub subscript: Option<Box<ParseNode>>,
}

use font::Symbol;
#[derive(Debug, PartialEq, Clone)]
pub struct Delimited {
    pub left:  Symbol,
    pub right: Symbol,
    pub inner: Vec<ParseNode>,
}

// TODO: It might be worth letting the `Group` variant
//   to have an atomtype associated with it.  By default,
//   it will be a `Ordinal`.

use spacing::Spacing;
#[derive(Debug, PartialEq, Clone)]
pub enum ParseNode {
    Symbol(Symbol),
    Delimited(Delimited),
    Group(Vec<ParseNode>),
    Radical(Radical),
    GenFraction(GenFraction),
    Scripts(Scripts),
    Spacing(Spacing),
    Rule(Rule),
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Rule {
    pub width: f64,
    pub height: f64,
}

use font::IsAtom;
impl IsAtom for ParseNode {
    fn atom_type(&self) -> Option<AtomType> {
        match *self {
            ParseNode::Symbol(ref sym) => Some(sym.atom_type),
            ParseNode::Group(_) => Some(AtomType::Ordinal),
            ParseNode::Delimited(_) => Some(AtomType::Fence),
            ParseNode::Radical(_) => Some(AtomType::Ordinal),
            ParseNode::GenFraction(_) => Some(AtomType::Inner),
            ParseNode::Scripts(_) => Some(AtomType::Ordinal), // Change to recursion
            ParseNode::Spacing(_) => None,
            ParseNode::Rule(_) => None,
        }
    } 
}

impl ParseNode {
    pub fn set_atom_type(&mut self, at: AtomType) {
        match *self {
            ParseNode::Symbol(ref mut sym) => sym.atom_type = at,
            _ => (),
        }
    }
}

// impl ParseNode {
//     fn has_superscript(&self) -> bool {
//         if let ParseNode::Scripts()
//     }
// 

#[derive(Debug, PartialEq, Clone)]
pub enum MathField {
    Symbol(Symbol),
    Group(Vec<ParseNode>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Radical {
    pub inner: Vec<ParseNode>,
    // We will handle optional arguments at a later day
    // pub superscript: Vec<ParseNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GenFraction {
    pub numerator: Vec<ParseNode>,
    pub denominator: Vec<ParseNode>,
    pub bar_thickness: u8,
    pub left_delimiter: Option<Symbol>,
    pub right_delimiter: Option<Symbol>,

}

// /// Every symbol will need a font family
// /// and a code point for how the symbol will be rendered.
// /// This font family and code point will be used to look up
// /// additional information regarding the dimensions if needed.

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub struct SymbolIdentifier {
//     pub font_family: u8,
//     pub code_point: u32,
// }

// #[derive(Debug)]
// pub struct Symbol {
//     pub font_family: u8,
//     pub code_point: u32, 
//     pub height: u32,
//     pub width: u32,
//     pub depth: u32,
//     pub atom_type: AtomType,
// }

// #[derive(Debug)]
// pub struct Operator {
//     pub symbol: Symbol,
//     pub limits: bool,
//     pub largeop: bool,
//     pub successor: Option<SymbolIdentifier>,
// }

// #[derive(Debug)]
// pub struct Delimiter {
//     pub symbol: Symbol,
//     pub successor: Option<SymbolIdentifier>,
// }

// // The following are wrapper types,
// // which mean they can wrap either a symbol
// // or mathlist.
// #[derive(Debug)]
// pub enum SymOrMathList {
//     Symbol(Symbol),
//     MathList(Box<MathList>),
// }

// #[derive(Debug)]
// pub struct Accent {
//     pub inner: SymOrMathList,
//     pub accent: Symbol,
//     pub kern: u32,
// }

// #[derive(Debug)]
// pub struct Scripts {
//     pub subscript: SymOrMathList,
//     pub supscript: SymOrMathList,
//     pub base:  SymOrMathList,
//     pub atom_type: AtomType,
// }

// #[derive(Debug)]
// pub enum ParseNode {
//     Symbol(Symbol),
//     Operator(Operator),
//     Delimiter(Delimiter),
//     Accent(Accent),
//     Radical(Radical),
//     Scripts(Scripts),
//     GenFraction(GenFraction),
// }