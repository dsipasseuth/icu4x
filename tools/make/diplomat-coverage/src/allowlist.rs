// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This file contains the allowlist for the ffi_coverage test
//!
//! Most new entries will go into IGNORED_PATHS

use std::collections::{HashMap, HashSet};

lazy_static::lazy_static! {
    pub static ref IGNORED_TRAITS: HashSet<&'static str> = [
        // Rust and Rust ecosystem types
        "Any",
        "AsMut",
        "AsRef",
        "Bake",
        "BakeSize",
        "Borrow",
        "BorrowMut",
        "Clone",
        "Copy",
        "Debug",
        "Default", // We expose this when we see fit
        "Deref", // We expose this when we see fit
        "Deserialize",
        "DeserializeOwned",
        "Display",
        "Eq",
        "ErasedDestructor",
        "Error",
        "From",
        "Hash",
        "Into",
        "IntoIterator", // skip IntoIterator but not Iterator
        "Ord",
        "PartialEq",
        "PartialOrd",
        "Provider", // From stdlib error infrastructure
        "RefUnwindSafe",
        "Send",
        "Separable",
        "Serialize",
        "StructuralEq",
        "StructuralPartialEq",
        "Sync",
        "ToOwned",
        "ToString", // We expose this when we see fit
        "TrieValue",
        "TryFrom", // We expose this when we see fit
        "TryInto", // We expose this when we see fit
        "Unpin",
        "UnwindSafe",

        // yoke/zerovec/etc internals
        "ULE",
        "AsULE",
        "VarULE",
        "Yokeable",
        "ZeroFrom",
        "ZeroMapKV",
        "EncodeAsULE",
        "EncodeAsVarULE",
        "IsCovariant",

        // provider stuff not relevant to FFI
        "DynamicDataMarker",
        "DataMarker",
        "AsDowncastingAnyProvider",
        "AsDeserializingBufferProvider",
        "AsDynamicDataProviderAnyMarkerWrap",
        "IterableDynamicDataProvider",
        "IterableDataProvider",
        "ForkByErrorPredicate",

        // The main data provider traits should be covered if the enum or struct
        // implementing them is covered
        "DataProvider",
        "DynamicDataProvider",
        "BufferProvider",
        "AnyProvider",

        // We might expose these if someone asks for it
        "DryDataProvider",
        "DynamicDryDataProvider",

        // internal trait, all methods replicated on Date
        "Calendar",
        // Rust-specific conversion trait
        "AsCalendar",
        "IntoAnyCalendar",
        "GetField",
    ].into_iter().collect();

    pub static ref IGNORED_ASSOCIATED_ITEMS: HashMap<&'static str, &'static [&'static str]> = [
        ("Writeable", &["writeable_length_hint", "write_to_parts", "write_to_string"][..]),
    ].into_iter().collect();

    // Ignore if this is a substring of any path
    // keep this small
    pub static ref IGNORED_SUBSTRINGS: &'static [&'static str] = &[
        // compiled data constructors cover these
        "_with_any_provider",
        // TODO-2.0 remove this
        "_with_buffer_provider",
        "_unstable",
        // Not planned for 2.0 but would be nice to return 'static refs
        // with Diplomat support.
        // Borrowed <-> owned converters
        "static_to_owned",
        "as_borrowed",
    ];
    // Paths which are not checked for FFI coverage. Naming a type or module here
    // will include all type methods and module contents.
    pub static ref IGNORED_PATHS: HashSet<Vec<String>> = [
        // Stuff that could be exposed over FFI but is not currently planned (for 2.0)
        //
        // Post 2.0 we should go through this and plan them, filing followups
        // for ones we do plan and adding links here
        // https://github.com/unicode-org/icu4x/issues/2492
        // =========================

        // Largely for use by datetimeformat, not generally useful
        "icu::calendar::AnyCalendar::convert_any_date",
        "icu::calendar::AnyCalendar::convert_any_datetime",
        "icu::calendar::Date::formattable_year",

        // Individual calendars: Currently the main entry point is AnyCalendar
        // We have chosen to not do individual calendars (except Iso) over FFI
        // since Diplomat can't do generics. We also support Gregorian *formatter*
        // but we don't need a separate Gregorian Date.
        "icu::calendar::cal",
        "icu::calendar::any_calendar::IntoAnyCalendar",
        "icu::calendar::Date::try_new_buddhist",
        "icu::calendar::Date::try_new_chinese_with_calendar",
        "icu::calendar::Date::try_new_coptic",
        "icu::calendar::Date::try_new_dangi",
        "icu::calendar::Date::try_new_dangi_with_calendar",
        "icu::calendar::Date::try_new_ethiopian",
        "icu::calendar::Date::try_new_gregorian",
        "icu::calendar::Date::try_new_hebrew",
        "icu::calendar::Date::try_new_hebrew_with_calendar",
        "icu::calendar::Date::try_new_indian",
        "icu::calendar::Date::try_new_islamic_civil_with_calendar",
        "icu::calendar::Date::try_new_islamic_tabular_with_calendar",
        "icu::calendar::Date::try_new_japanese_with_calendar",
        "icu::calendar::Date::try_new_japanese_extended_with_calendar",
        "icu::calendar::Date::try_new_julian",
        "icu::calendar::Date::try_new_observational_islamic_with_calendar",
        "icu::calendar::Date::try_new_persian",
        "icu::calendar::Date::try_new_roc",
        "icu::calendar::Date::try_new_ummalqura_with_calendar",
        "icu::calendar::DateTime::try_new_buddhist",
        "icu::calendar::DateTime::try_new_chinese_with_calendar",
        "icu::calendar::DateTime::try_new_coptic",
        "icu::calendar::DateTime::try_new_dangi",
        "icu::calendar::DateTime::try_new_dangi_with_calendar",
        "icu::calendar::DateTime::try_new_ethiopian",
        "icu::calendar::DateTime::try_new_gregorian",
        "icu::calendar::DateTime::try_new_hebrew",
        "icu::calendar::DateTime::try_new_hebrew_with_calendar",
        "icu::calendar::DateTime::try_new_indian",
        "icu::calendar::DateTime::try_new_islamic_civil_with_calendar",
        "icu::calendar::DateTime::try_new_islamic_tabular_with_calendar",
        "icu::calendar::DateTime::try_new_japanese_with_calendar",
        "icu::calendar::DateTime::try_new_japanese_extended_with_calendar",
        "icu::calendar::DateTime::try_new_julian",
        "icu::calendar::DateTime::try_new_observational_islamic_with_calendar",
        "icu::calendar::DateTime::try_new_persian",
        "icu::calendar::DateTime::try_new_roc",
        "icu::calendar::DateTime::try_new_ummalqura_with_calendar",

        // Calendar structs mostly for internal use but which might expose
        // useful information to clients.
        "icu::calendar::types::MonthInfo",
        "icu::calendar::types::FormattingEra",
        "icu::calendar::types::FormattableYear",
        "icu::calendar::types::FormattableYearKind",
        "icu::calendar::types::DayOfYearInfo",

        // Not yet fully exposed over FFI, Temporal doesn't yet want this.
        "icu::calendar::types::CyclicYear",
        "icu::calendar::types::YearInfo::cyclic",
        "icu::calendar::types::YearInfo::related_iso",

        // Experimental API mostly used for provider, components bags, and patterns,
        // may in the future be exposed for options
        "icu::datetime::fields",

        // experimental
        "icu::datetime::fields::components",
        "icu::datetime::fieldsets::serde",
        "icu::datetime::neo_pattern",
        "icu::datetime::neo_skeleton",
        "icu::datetime::options::components",
        "icu::datetime::options::preferences",
        "icu::datetime::DateTimeWriteError",
        "icu::datetime::FormattedDateTimePattern",
        "icu::datetime::pattern::TypedDateTimeNames",

        // experimental
        "icu::experimental",

        // Experimental and unused decimal types
        "fixed_decimal::CompactDecimal",
        "fixed_decimal::FixedInteger",
        "fixed_decimal::ScientificDecimal",

        // Don't want parts for 2.0, would need to introduce diplomat writeable with parts
        "icu::list::parts",

        // Not planned until someone needs them
        "icu::locale::extensions",
        "icu::locale::subtags",

        // TODO-2.0: decide later when we have figured out prefs/ctors and have APIs using this
        "icu::locale::LanguageIdentifier",

        // Doesn't make sense to expose through `icu_normalizer`
        "icu::normalizer::uts46::Uts46Mapper",

        // Do not want for 2.0: we need DiplomatWriteable16
        "icu::normalizer::ComposingNormalizer::normalize_utf16",
        "icu::normalizer::ComposingNormalizer::normalize_utf16_to",
        "icu::normalizer::DecomposingNormalizer::normalize_utf16",
        "icu::normalizer::DecomposingNormalizer::normalize_utf16_to",

        // Do not want for 2.0:
        // Can't be exposed till diplomat has input iterators, as well as
        // safety for borrowing input iterators into return types
        "icu::normalizer::ComposingNormalizer::normalize_iter",
        "icu::normalizer::DecomposingNormalizer::normalize_iter",
        "icu::normalizer::Composition",
        "icu::normalizer::Decomposition",

        // experimental
        "icu::plurals::rules",

        // Experimental
        "icu::plurals::PluralRulesWithRanges",
        "icu::plurals::PluralRulesWithRanges::categories",
        "icu::plurals::PluralRulesWithRanges::category_for",
        "icu::plurals::PluralRulesWithRanges::category_for_range",
        "icu::plurals::PluralRulesWithRanges::resolve_range",
        "icu::plurals::PluralRulesWithRanges::try_new",
        "icu::plurals::PluralRulesWithRanges::try_new_cardinal",
        "icu::plurals::PluralRulesWithRanges::try_new_ordinal",

        // Not planned for 2.0
        // We aren't exposing these collections directly, we instead expose them in a domain specific
        // way like CodePointSetDataBuilder. We may eventually add these as utilities for users.
        "icu::collections",
        "icu::properties::CodePointMapData::as_code_point_trie",
        "icu::properties::CodePointMapData::from_code_point_trie",
        "icu::properties::CodePointMapData::to_code_point_trie",
        "icu::properties::CodePointMapDataBorrowed::iter_ranges",
        "icu::properties::EmojiSetData::as_code_point_inversion_list_string_list",
        "icu::properties::EmojiSetData::from_code_point_inversion_list_string_list",
        "icu::properties::EmojiSetData::to_code_point_inversion_list_string_list",

        // We do not plan to have FFI for this in 2.0
        "icu_provider_adapters::empty::EmptyDataProvider",

        // We should add this once we have a better story for FFI custom data structs
        // and callbacks
        "icu_provider_adapters::fixed::FixedProvider",

        // Not planned for 2.0
        // FilterDataProvider::with_filter needs callbacks.
        "icu_provider_adapters::filter::FilterDataProvider",

        // Not planned for 2.0
        // ForkByErrorProvider is the abstract forking provider; we expose the concrete
        // fork by locale/key ones. Could be added if we have callbacks.
        "icu_provider_adapters::fork::ForkByErrorProvider",
        "icu_provider_adapters::fork::predicates::ForkByErrorPredicate",

        // Stuff that does not need to be exposed over FFI
        // Especially for stuff that are Rust specific like conversion traits
        // and markers and newtypes
        // =========================

        // Datagen
        "icu::markers_for_bin",

        // Scaffolding modules
        "icu::datetime::scaffold",
        "icu::timezone::scaffold",

        // Provider modules
        // We could potentially expose them later, but it's hard to expose them
        // uniformly especially for complex types
        "icu::calendar::provider",
        "icu::casemap::provider",
        "icu::collator::provider",
        "icu::datetime::provider",
        "icu::decimal::provider",
        "icu::list::provider",
        "icu::locale::provider",
        "icu::normalizer::provider",
        "icu::plurals::provider",
        "icu::properties::provider",
        "icu::segmenter::provider",
        "icu::timezone::provider",
        "icu::transliterate::provider",

        // ULE types that are not in provider modules
        "icu::collections::codepointinvlist::CodePointInversionListULE",
        "icu::plurals::PluralCategoryULE",

        // Reexported
        "icu::calendar::any_calendar::AnyCalendar",
        "icu::calendar::any_calendar::AnyCalendarKind",
        "icu::casemap::titlecase::TitlecaseMapper",
        "icu::calendar::types::Time",

        // "Internal" trait that should never be called directly
        "icu::calendar::Calendar",

        // Rust-specific calendar wrapper stuff
        "icu::calendar::AsCalendar",
        "icu::calendar::Ref",
        "icu::datetime::CldrCalendar",
        // TODO-2.0: needs investigation
        "icu::calendar::Date::wrap_calendar_in_rc",
        "icu::calendar::Date::wrap_calendar_in_arc",
        "icu::calendar::DateTime::wrap_calendar_in_rc",
        "icu::calendar::DateTime::wrap_calendar_in_arc",

        // Individual markerlike calendar types and inner types
        // inner types are only public for associated type reasons, and the markerlike
        // calendar types exist to implement the trait
        "icu::calendar::Date::from_raw",
        "icu::calendar::Date::inner",
        "icu::calendar::Iso",
        "icu::calendar::cal::Iso",
        "icu::calendar::cal::IsoDateInner",
        "icu::calendar::Gregorian",
        "icu::calendar::cal::Gregorian",
        "icu::calendar::cal::GregorianDateInner",
        "icu::calendar::any_calendar::AnyDateInner",

        // Options bags which are expanded in FFI to regular functions
        // TODO-2.0: investigate flattening on the rust side too
        "icu::datetime::DateTimeFormatterOptions",
        "icu::datetime::options::DateTimeFormatterOptions",
        "icu::datetime::options::length::Bag",
        "icu::decimal::options::FixedDecimalFormatterOptions",

        // FFI largely deals with primitives rather than Rust's nice wrapper types
        // (which are hard to do in a zero-cost way over FFI)
        "icu::calendar::types::MonthCode",
        "icu::calendar::types::DayOfMonth",
        "icu::calendar::types::WeekOfMonth",
        "icu::calendar::types::WeekOfYear",
        "icu::calendar::types::DayOfWeekInMonth",
        "icu::calendar::types::IsoHour",
        "icu::calendar::types::IsoMinute",
        "icu::calendar::types::IsoSecond",
        "icu::calendar::types::NanoSecond",
        "icu::calendar::types::IsoWeekday",
        "icu::calendar::types::Era",

        // Convenience iterator for Rust. Useful but would require
        // allocations over FFI, so not worth it.
        "icu::plurals::PluralCategory::all",

        // Some of the provider adapter types are Rust-specific and not relevant to FFI
        "icu_provider_adapters::either::EitherProvider",

        // Decompositions of providers is tricky to do over FFI and the use cases are unclear.
        "icu_provider_adapters::fallback::LocaleFallbackProvider::inner",
        "icu_provider_adapters::fallback::LocaleFallbackProvider::into_inner",
        "icu_provider_adapters::fallback::LocaleFallbackProvider::inner_mut",

        // The polymorphic ICU4XDataProvider type makes the MultiFork providers less relevant.
        "icu_provider_adapters::fork::MultiForkByErrorProvider",
        "icu_provider_adapters::fork::MultiForkByMarkerProvider",

        // Specialized constructor for separately constructed instances
        "icu::timezone::TimeZoneIdMapperWithFastCanonicalization::try_new_with_mapper",

        // macros
        "icu::locale::langid",
        "icu::locale::locale",
        "icu::locale::extensions::other::subtag",
        "icu::locale::extensions::private::subtag",
        "icu::locale::extensions::transform::key",
        "icu::locale::extensions::unicode::attribute",
        "icu::locale::extensions::unicode::key",
        "icu::locale::extensions::unicode::value",
        "icu::locale::subtags::language",
        "icu::locale::subtags::region",
        "icu::locale::subtags::script",
        "icu::locale::subtags::variant",
        "icu_provider_adapters::make_forking_provider",

        // assoc types
        "icu::locale::Locale::Err",
        "icu::plurals::PluralOperands::Err",

    ].iter().map(|s| s.split("::").map(str::to_owned).collect()).collect();
}
