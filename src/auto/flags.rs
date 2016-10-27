// This file was generated by gir (d9591be+) from gir-files (???)
// DO NOT EDIT

use ffi;
use glib::translate::*;

#[cfg(feature = "v2_10")]
bitflags! {
    pub flags EditorTypingAttributes: u32 {
        const EDITOR_TYPING_ATTRIBUTE_NONE = 2,
        const EDITOR_TYPING_ATTRIBUTE_BOLD = 4,
        const EDITOR_TYPING_ATTRIBUTE_ITALIC = 8,
        const EDITOR_TYPING_ATTRIBUTE_UNDERLINE = 16,
        const EDITOR_TYPING_ATTRIBUTE_STRIKETHROUGH = 32,
    }
}

#[cfg(feature = "v2_10")]
#[doc(hidden)]
impl ToGlib for EditorTypingAttributes {
    type GlibType = ffi::WebKitEditorTypingAttributes;

    fn to_glib(&self) -> ffi::WebKitEditorTypingAttributes {
        ffi::WebKitEditorTypingAttributes::from_bits_truncate(self.bits())
    }
}

#[cfg(feature = "v2_10")]
#[doc(hidden)]
impl FromGlib<ffi::WebKitEditorTypingAttributes> for EditorTypingAttributes {
    fn from_glib(value: ffi::WebKitEditorTypingAttributes) -> EditorTypingAttributes {
        skip_assert_initialized!();
        EditorTypingAttributes::from_bits_truncate(value.bits())
    }
}

bitflags! {
    pub flags FindOptions: u32 {
        const FIND_OPTIONS_NONE = 0,
        const FIND_OPTIONS_CASE_INSENSITIVE = 1,
        const FIND_OPTIONS_AT_WORD_STARTS = 2,
        const FIND_OPTIONS_TREAT_MEDIAL_CAPITAL_AS_WORD_START = 4,
        const FIND_OPTIONS_BACKWARDS = 8,
        const FIND_OPTIONS_WRAP_AROUND = 16,
    }
}

#[doc(hidden)]
impl ToGlib for FindOptions {
    type GlibType = ffi::WebKitFindOptions;

    fn to_glib(&self) -> ffi::WebKitFindOptions {
        ffi::WebKitFindOptions::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WebKitFindOptions> for FindOptions {
    fn from_glib(value: ffi::WebKitFindOptions) -> FindOptions {
        skip_assert_initialized!();
        FindOptions::from_bits_truncate(value.bits())
    }
}

bitflags! {
    pub flags HitTestResultContext: u32 {
        const HIT_TEST_RESULT_CONTEXT_DOCUMENT = 2,
        const HIT_TEST_RESULT_CONTEXT_LINK = 4,
        const HIT_TEST_RESULT_CONTEXT_IMAGE = 8,
        const HIT_TEST_RESULT_CONTEXT_MEDIA = 16,
        const HIT_TEST_RESULT_CONTEXT_EDITABLE = 32,
        const HIT_TEST_RESULT_CONTEXT_SCROLLBAR = 64,
        const HIT_TEST_RESULT_CONTEXT_SELECTION = 128,
    }
}

#[doc(hidden)]
impl ToGlib for HitTestResultContext {
    type GlibType = ffi::WebKitHitTestResultContext;

    fn to_glib(&self) -> ffi::WebKitHitTestResultContext {
        ffi::WebKitHitTestResultContext::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WebKitHitTestResultContext> for HitTestResultContext {
    fn from_glib(value: ffi::WebKitHitTestResultContext) -> HitTestResultContext {
        skip_assert_initialized!();
        HitTestResultContext::from_bits_truncate(value.bits())
    }
}

bitflags! {
    pub flags SnapshotOptions: u32 {
        const SNAPSHOT_OPTIONS_NONE = 0,
        const SNAPSHOT_OPTIONS_INCLUDE_SELECTION_HIGHLIGHTING = 1,
        const SNAPSHOT_OPTIONS_TRANSPARENT_BACKGROUND = 2,
    }
}

#[doc(hidden)]
impl ToGlib for SnapshotOptions {
    type GlibType = ffi::WebKitSnapshotOptions;

    fn to_glib(&self) -> ffi::WebKitSnapshotOptions {
        ffi::WebKitSnapshotOptions::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WebKitSnapshotOptions> for SnapshotOptions {
    fn from_glib(value: ffi::WebKitSnapshotOptions) -> SnapshotOptions {
        skip_assert_initialized!();
        SnapshotOptions::from_bits_truncate(value.bits())
    }
}

