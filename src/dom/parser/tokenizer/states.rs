use crate::utils::SharedStr;

// #[derive(Debug, Clone)]
// pub enum State {
//     Data,
//     // Plaintext,
//     TagOpen,
//     EndTagOpen,
//     TagName,
//     // RawData(RawKind),
//     // RawLessThanSign(RawKind),
//     // RawEndTagOpen(RawKind),
//     // RawEndTagName(RawKind),
//
//     // ScriptDataEscapeStart(ScriptEscapeKind),
//     // ScriptDataEscapeStartDash,
//     // ScriptDataEscapedDash(ScriptEscapeKind),
//     // ScriptDataEscapedDashDash(ScriptEscapeKind),
//     // ScriptDataDoubleEscapeEnd,
//     SpecialTag(SharedStr),
//     SpecialEndTagOpen(SharedStr),
//     SpecialEndTag(SharedStr),
//     BeforeAttributeName,
//     AttributeName,
//     AfterAttributeName,
//     BeforeAttributeValue,
//     AttributeValue(AttrValueKind),
//     AfterAttributeValueQuoted,
//     SelfClosingStartTag,
//     BogusComment,
//     MarkupDeclarationOpen,
//     CommentStart,
//     CommentStartDash,
//     Comment,
//     // CommentLessThanSign,
//     // CommentLessThanSignBang,
//     // CommentLessThanSignBangDash,
//     // CommentLessThanSignBangDashDash,
//     CommentEndDash,
//     CommentEnd,
//     CommentEndBang,
//     Doctype,
//     BeforeDoctypeName,
//     DoctypeName,
//     AfterDoctypeName,
//     // TODO possibly remove
//     // AfterDoctypeKeyword(DoctypeIdKind),
//     // BeforeDoctypeIdentifier(DoctypeIdKind),
//     // DoctypeIdentifierDoubleQuoted(DoctypeIdKind),
//     // DoctypeIdentifierSingleQuoted(DoctypeIdKind),
//     // AfterDoctypeIdentifier(DoctypeIdKind),
//     // BetweenDoctypePublicAndSystemIdentifiers,
//     // BogusDoctype,
//     // CdataSection,
//     // CdataSectionBracket,
//     // CdataSectionEnd,
// }