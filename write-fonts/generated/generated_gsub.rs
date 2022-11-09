// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// [GSUB](https://learn.microsoft.com/en-us/typography/opentype/spec/gsub#gsub-header)
#[derive(Clone, Debug, Default)]
pub struct Gsub {
    /// Offset to ScriptList table, from beginning of GSUB table
    pub script_list: OffsetMarker<ScriptList>,
    /// Offset to FeatureList table, from beginning of GSUB table
    pub feature_list: OffsetMarker<FeatureList>,
    /// Offset to LookupList table, from beginning of GSUB table
    pub lookup_list: OffsetMarker<SubstitutionLookupList>,
    /// Offset to FeatureVariations table, from beginning of the GSUB
    /// table (may be NULL)
    pub feature_variations: NullableOffsetMarker<FeatureVariations, WIDTH_32>,
}

impl Gsub {
    /// Construct a new `Gsub`
    pub fn new(
        script_list: ScriptList,
        feature_list: FeatureList,
        lookup_list: SubstitutionLookupList,
    ) -> Self {
        Self {
            script_list: script_list.into(),
            feature_list: feature_list.into(),
            lookup_list: lookup_list.into(),
            ..Default::default()
        }
    }
}

impl FontWrite for Gsub {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        let version = self.compute_version() as MajorMinor;
        version.write_into(writer);
        self.script_list.write_into(writer);
        self.feature_list.write_into(writer);
        self.lookup_list.write_into(writer);
        version
            .compatible((1, 1))
            .then(|| self.feature_variations.write_into(writer));
    }
    fn table_type(&self) -> TableType {
        TableType::TopLevel(Gsub::TAG)
    }
}

impl Validate for Gsub {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("Gsub", |ctx| {
            ctx.in_field("script_list", |ctx| {
                self.script_list.validate_impl(ctx);
            });
            ctx.in_field("feature_list", |ctx| {
                self.feature_list.validate_impl(ctx);
            });
            ctx.in_field("lookup_list", |ctx| {
                self.lookup_list.validate_impl(ctx);
            });
            ctx.in_field("feature_variations", |ctx| {
                self.feature_variations.validate_impl(ctx);
            });
        })
    }
}

impl TopLevelTable for Gsub {
    const TAG: Tag = Tag::new(b"GSUB");
}

impl<'a> FromObjRef<read_fonts::tables::gsub::Gsub<'a>> for Gsub {
    fn from_obj_ref(obj: &read_fonts::tables::gsub::Gsub<'a>, _: FontData) -> Self {
        Gsub {
            script_list: obj.script_list().to_owned_table(),
            feature_list: obj.feature_list().to_owned_table(),
            lookup_list: obj.lookup_list().to_owned_table(),
            feature_variations: obj.feature_variations().to_owned_table(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::gsub::Gsub<'a>> for Gsub {}

impl<'a> FontRead<'a> for Gsub {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::gsub::Gsub as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

/// A [GSUB Lookup](https://learn.microsoft.com/en-us/typography/opentype/spec/gsub#gsubLookupTypeEnum) subtable.
#[derive(Debug, Clone)]
pub enum SubstitutionLookup {
    Single(Lookup<SingleSubst>),
    Multiple(Lookup<MultipleSubstFormat1>),
    Alternate(Lookup<AlternateSubstFormat1>),
    Ligature(Lookup<LigatureSubstFormat1>),
    Contextual(Lookup<SubstitutionSequenceContext>),
    ChainContextual(Lookup<SubstitutionChainContext>),
    Extension(Lookup<ExtensionSubtable>),
    Reverse(Lookup<ReverseChainSingleSubstFormat1>),
}

impl Default for SubstitutionLookup {
    fn default() -> Self {
        Self::Single(Default::default())
    }
}

impl FontWrite for SubstitutionLookup {
    fn write_into(&self, writer: &mut TableWriter) {
        match self {
            Self::Single(table) => table.write_into(writer),
            Self::Multiple(table) => table.write_into(writer),
            Self::Alternate(table) => table.write_into(writer),
            Self::Ligature(table) => table.write_into(writer),
            Self::Contextual(table) => table.write_into(writer),
            Self::ChainContextual(table) => table.write_into(writer),
            Self::Extension(table) => table.write_into(writer),
            Self::Reverse(table) => table.write_into(writer),
        }
    }
    fn table_type(&self) -> TableType {
        match self {
            Self::Single(table) => table.table_type(),
            Self::Multiple(table) => table.table_type(),
            Self::Alternate(table) => table.table_type(),
            Self::Ligature(table) => table.table_type(),
            Self::Contextual(table) => table.table_type(),
            Self::ChainContextual(table) => table.table_type(),
            Self::Extension(table) => table.table_type(),
            Self::Reverse(table) => table.table_type(),
        }
    }
}

impl Validate for SubstitutionLookup {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        match self {
            Self::Single(table) => table.validate_impl(ctx),
            Self::Multiple(table) => table.validate_impl(ctx),
            Self::Alternate(table) => table.validate_impl(ctx),
            Self::Ligature(table) => table.validate_impl(ctx),
            Self::Contextual(table) => table.validate_impl(ctx),
            Self::ChainContextual(table) => table.validate_impl(ctx),
            Self::Extension(table) => table.validate_impl(ctx),
            Self::Reverse(table) => table.validate_impl(ctx),
        }
    }
}

impl FromObjRef<read_fonts::tables::gsub::SubstitutionLookup<'_>> for SubstitutionLookup {
    fn from_obj_ref(
        from: &read_fonts::tables::gsub::SubstitutionLookup<'_>,
        data: FontData,
    ) -> Self {
        match from {
            read_fonts::tables::gsub::SubstitutionLookup::Single(table) => {
                Self::Single(table.to_owned_obj(data))
            }
            read_fonts::tables::gsub::SubstitutionLookup::Multiple(table) => {
                Self::Multiple(table.to_owned_obj(data))
            }
            read_fonts::tables::gsub::SubstitutionLookup::Alternate(table) => {
                Self::Alternate(table.to_owned_obj(data))
            }
            read_fonts::tables::gsub::SubstitutionLookup::Ligature(table) => {
                Self::Ligature(table.to_owned_obj(data))
            }
            read_fonts::tables::gsub::SubstitutionLookup::Contextual(table) => {
                Self::Contextual(table.to_owned_obj(data))
            }
            read_fonts::tables::gsub::SubstitutionLookup::ChainContextual(table) => {
                Self::ChainContextual(table.to_owned_obj(data))
            }
            read_fonts::tables::gsub::SubstitutionLookup::Extension(table) => {
                Self::Extension(table.to_owned_obj(data))
            }
            read_fonts::tables::gsub::SubstitutionLookup::Reverse(table) => {
                Self::Reverse(table.to_owned_obj(data))
            }
        }
    }
}

impl FromTableRef<read_fonts::tables::gsub::SubstitutionLookup<'_>> for SubstitutionLookup {}

/// LookupType 1: [Single Substitution](https://learn.microsoft.com/en-us/typography/opentype/spec/gsub#lookuptype-1-single-substitution-subtable) Subtable
#[derive(Clone, Debug)]
pub enum SingleSubst {
    Format1(SingleSubstFormat1),
    Format2(SingleSubstFormat2),
}

impl SingleSubst {
    /// Construct a new `SingleSubstFormat1` subtable
    pub fn format_1(coverage: CoverageTable, delta_glyph_id: i16) -> Self {
        Self::Format1(SingleSubstFormat1::new(coverage, delta_glyph_id))
    }

    /// Construct a new `SingleSubstFormat2` subtable
    pub fn format_2(coverage: CoverageTable, substitute_glyph_ids: Vec<GlyphId>) -> Self {
        Self::Format2(SingleSubstFormat2::new(coverage, substitute_glyph_ids))
    }
}

impl Default for SingleSubst {
    fn default() -> Self {
        Self::Format1(Default::default())
    }
}

impl FontWrite for SingleSubst {
    fn write_into(&self, writer: &mut TableWriter) {
        match self {
            Self::Format1(item) => item.write_into(writer),
            Self::Format2(item) => item.write_into(writer),
        }
    }
    fn table_type(&self) -> TableType {
        match self {
            Self::Format1(item) => item.table_type(),
            Self::Format2(item) => item.table_type(),
        }
    }
}

impl Validate for SingleSubst {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        match self {
            Self::Format1(item) => item.validate_impl(ctx),
            Self::Format2(item) => item.validate_impl(ctx),
        }
    }
}

impl FromObjRef<read_fonts::tables::gsub::SingleSubst<'_>> for SingleSubst {
    fn from_obj_ref(obj: &read_fonts::tables::gsub::SingleSubst, _: FontData) -> Self {
        use read_fonts::tables::gsub::SingleSubst as ObjRefType;
        match obj {
            ObjRefType::Format1(item) => SingleSubst::Format1(item.to_owned_table()),
            ObjRefType::Format2(item) => SingleSubst::Format2(item.to_owned_table()),
        }
    }
}

impl FromTableRef<read_fonts::tables::gsub::SingleSubst<'_>> for SingleSubst {}

impl<'a> FontRead<'a> for SingleSubst {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::gsub::SingleSubst as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

/// [Single Substitution Format 1](https://learn.microsoft.com/en-us/typography/opentype/spec/gsub#11-single-substitution-format-1)
#[derive(Clone, Debug, Default)]
pub struct SingleSubstFormat1 {
    /// Offset to Coverage table, from beginning of substitution
    /// subtable
    pub coverage: OffsetMarker<CoverageTable>,
    /// Add to original glyph ID to get substitute glyph ID
    pub delta_glyph_id: i16,
}

impl SingleSubstFormat1 {
    /// Construct a new `SingleSubstFormat1`
    pub fn new(coverage: CoverageTable, delta_glyph_id: i16) -> Self {
        Self {
            coverage: coverage.into(),
            delta_glyph_id,
        }
    }
}

impl FontWrite for SingleSubstFormat1 {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (1 as u16).write_into(writer);
        self.coverage.write_into(writer);
        self.delta_glyph_id.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("SingleSubstFormat1")
    }
}

impl Validate for SingleSubstFormat1 {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("SingleSubstFormat1", |ctx| {
            ctx.in_field("coverage", |ctx| {
                self.coverage.validate_impl(ctx);
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::tables::gsub::SingleSubstFormat1<'a>> for SingleSubstFormat1 {
    fn from_obj_ref(obj: &read_fonts::tables::gsub::SingleSubstFormat1<'a>, _: FontData) -> Self {
        SingleSubstFormat1 {
            coverage: obj.coverage().to_owned_table(),
            delta_glyph_id: obj.delta_glyph_id(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::gsub::SingleSubstFormat1<'a>> for SingleSubstFormat1 {}

impl<'a> FontRead<'a> for SingleSubstFormat1 {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::gsub::SingleSubstFormat1 as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}

/// [Single Substitution Format 2](https://learn.microsoft.com/en-us/typography/opentype/spec/gsub#12-single-substitution-format-2)
#[derive(Clone, Debug, Default)]
pub struct SingleSubstFormat2 {
    /// Offset to Coverage table, from beginning of substitution
    /// subtable
    pub coverage: OffsetMarker<CoverageTable>,
    /// Array of substitute glyph IDs — ordered by Coverage index
    pub substitute_glyph_ids: Vec<GlyphId>,
}

impl SingleSubstFormat2 {
    /// Construct a new `SingleSubstFormat2`
    pub fn new(coverage: CoverageTable, substitute_glyph_ids: Vec<GlyphId>) -> Self {
        Self {
            coverage: coverage.into(),
            substitute_glyph_ids: substitute_glyph_ids.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for SingleSubstFormat2 {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (2 as u16).write_into(writer);
        self.coverage.write_into(writer);
        (array_len(&self.substitute_glyph_ids).unwrap() as u16).write_into(writer);
        self.substitute_glyph_ids.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("SingleSubstFormat2")
    }
}

impl Validate for SingleSubstFormat2 {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("SingleSubstFormat2", |ctx| {
            ctx.in_field("coverage", |ctx| {
                self.coverage.validate_impl(ctx);
            });
            ctx.in_field("substitute_glyph_ids", |ctx| {
                if self.substitute_glyph_ids.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::tables::gsub::SingleSubstFormat2<'a>> for SingleSubstFormat2 {
    fn from_obj_ref(obj: &read_fonts::tables::gsub::SingleSubstFormat2<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        SingleSubstFormat2 {
            coverage: obj.coverage().to_owned_table(),
            substitute_glyph_ids: obj.substitute_glyph_ids().to_owned_obj(offset_data),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::gsub::SingleSubstFormat2<'a>> for SingleSubstFormat2 {}

impl<'a> FontRead<'a> for SingleSubstFormat2 {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::gsub::SingleSubstFormat2 as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}

/// [Multiple Substitution Format 1](https://learn.microsoft.com/en-us/typography/opentype/spec/gsub#21-multiple-substitution-format-1)
#[derive(Clone, Debug, Default)]
pub struct MultipleSubstFormat1 {
    /// Offset to Coverage table, from beginning of substitution
    /// subtable
    pub coverage: OffsetMarker<CoverageTable>,
    /// Array of offsets to Sequence tables. Offsets are from beginning
    /// of substitution subtable, ordered by Coverage index
    pub sequences: Vec<OffsetMarker<Sequence>>,
}

impl MultipleSubstFormat1 {
    /// Construct a new `MultipleSubstFormat1`
    pub fn new(coverage: CoverageTable, sequences: Vec<Sequence>) -> Self {
        Self {
            coverage: coverage.into(),
            sequences: sequences.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for MultipleSubstFormat1 {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (1 as u16).write_into(writer);
        self.coverage.write_into(writer);
        (array_len(&self.sequences).unwrap() as u16).write_into(writer);
        self.sequences.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("MultipleSubstFormat1")
    }
}

impl Validate for MultipleSubstFormat1 {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("MultipleSubstFormat1", |ctx| {
            ctx.in_field("coverage", |ctx| {
                self.coverage.validate_impl(ctx);
            });
            ctx.in_field("sequences", |ctx| {
                if self.sequences.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
                self.sequences.validate_impl(ctx);
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::tables::gsub::MultipleSubstFormat1<'a>> for MultipleSubstFormat1 {
    fn from_obj_ref(obj: &read_fonts::tables::gsub::MultipleSubstFormat1<'a>, _: FontData) -> Self {
        MultipleSubstFormat1 {
            coverage: obj.coverage().to_owned_table(),
            sequences: obj.sequences().to_owned_table(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::gsub::MultipleSubstFormat1<'a>> for MultipleSubstFormat1 {}

impl<'a> FontRead<'a> for MultipleSubstFormat1 {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::gsub::MultipleSubstFormat1 as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}

/// Part of [MultipleSubstFormat1]
#[derive(Clone, Debug, Default)]
pub struct Sequence {
    /// String of glyph IDs to substitute
    pub substitute_glyph_ids: Vec<GlyphId>,
}

impl Sequence {
    /// Construct a new `Sequence`
    pub fn new(substitute_glyph_ids: Vec<GlyphId>) -> Self {
        Self {
            substitute_glyph_ids: substitute_glyph_ids.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for Sequence {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (array_len(&self.substitute_glyph_ids).unwrap() as u16).write_into(writer);
        self.substitute_glyph_ids.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("Sequence")
    }
}

impl Validate for Sequence {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("Sequence", |ctx| {
            ctx.in_field("substitute_glyph_ids", |ctx| {
                if self.substitute_glyph_ids.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::tables::gsub::Sequence<'a>> for Sequence {
    fn from_obj_ref(obj: &read_fonts::tables::gsub::Sequence<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        Sequence {
            substitute_glyph_ids: obj.substitute_glyph_ids().to_owned_obj(offset_data),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::gsub::Sequence<'a>> for Sequence {}

impl<'a> FontRead<'a> for Sequence {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::gsub::Sequence as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

/// [Alternate Substitution Format 1](https://learn.microsoft.com/en-us/typography/opentype/spec/gsub#31-alternate-substitution-format-1)
#[derive(Clone, Debug, Default)]
pub struct AlternateSubstFormat1 {
    /// Offset to Coverage table, from beginning of substitution
    /// subtable
    pub coverage: OffsetMarker<CoverageTable>,
    /// Array of offsets to AlternateSet tables. Offsets are from
    /// beginning of substitution subtable, ordered by Coverage index
    pub alternate_sets: Vec<OffsetMarker<AlternateSet>>,
}

impl AlternateSubstFormat1 {
    /// Construct a new `AlternateSubstFormat1`
    pub fn new(coverage: CoverageTable, alternate_sets: Vec<AlternateSet>) -> Self {
        Self {
            coverage: coverage.into(),
            alternate_sets: alternate_sets.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for AlternateSubstFormat1 {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (1 as u16).write_into(writer);
        self.coverage.write_into(writer);
        (array_len(&self.alternate_sets).unwrap() as u16).write_into(writer);
        self.alternate_sets.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("AlternateSubstFormat1")
    }
}

impl Validate for AlternateSubstFormat1 {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("AlternateSubstFormat1", |ctx| {
            ctx.in_field("coverage", |ctx| {
                self.coverage.validate_impl(ctx);
            });
            ctx.in_field("alternate_sets", |ctx| {
                if self.alternate_sets.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
                self.alternate_sets.validate_impl(ctx);
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::tables::gsub::AlternateSubstFormat1<'a>> for AlternateSubstFormat1 {
    fn from_obj_ref(
        obj: &read_fonts::tables::gsub::AlternateSubstFormat1<'a>,
        _: FontData,
    ) -> Self {
        AlternateSubstFormat1 {
            coverage: obj.coverage().to_owned_table(),
            alternate_sets: obj.alternate_sets().to_owned_table(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::gsub::AlternateSubstFormat1<'a>>
    for AlternateSubstFormat1
{
}

impl<'a> FontRead<'a> for AlternateSubstFormat1 {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::gsub::AlternateSubstFormat1 as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}

/// Part of [AlternateSubstFormat1]
#[derive(Clone, Debug, Default)]
pub struct AlternateSet {
    /// Array of alternate glyph IDs, in arbitrary order
    pub alternate_glyph_ids: Vec<GlyphId>,
}

impl AlternateSet {
    /// Construct a new `AlternateSet`
    pub fn new(alternate_glyph_ids: Vec<GlyphId>) -> Self {
        Self {
            alternate_glyph_ids: alternate_glyph_ids.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for AlternateSet {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (array_len(&self.alternate_glyph_ids).unwrap() as u16).write_into(writer);
        self.alternate_glyph_ids.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("AlternateSet")
    }
}

impl Validate for AlternateSet {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("AlternateSet", |ctx| {
            ctx.in_field("alternate_glyph_ids", |ctx| {
                if self.alternate_glyph_ids.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::tables::gsub::AlternateSet<'a>> for AlternateSet {
    fn from_obj_ref(obj: &read_fonts::tables::gsub::AlternateSet<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        AlternateSet {
            alternate_glyph_ids: obj.alternate_glyph_ids().to_owned_obj(offset_data),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::gsub::AlternateSet<'a>> for AlternateSet {}

impl<'a> FontRead<'a> for AlternateSet {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::gsub::AlternateSet as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

/// [Ligature Substitution Format 1](https://learn.microsoft.com/en-us/typography/opentype/spec/gsub#41-ligature-substitution-format-1)
#[derive(Clone, Debug, Default)]
pub struct LigatureSubstFormat1 {
    /// Offset to Coverage table, from beginning of substitution
    /// subtable
    pub coverage: OffsetMarker<CoverageTable>,
    /// Array of offsets to LigatureSet tables. Offsets are from
    /// beginning of substitution subtable, ordered by Coverage index
    pub ligature_sets: Vec<OffsetMarker<LigatureSet>>,
}

impl LigatureSubstFormat1 {
    /// Construct a new `LigatureSubstFormat1`
    pub fn new(coverage: CoverageTable, ligature_sets: Vec<LigatureSet>) -> Self {
        Self {
            coverage: coverage.into(),
            ligature_sets: ligature_sets.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for LigatureSubstFormat1 {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (1 as u16).write_into(writer);
        self.coverage.write_into(writer);
        (array_len(&self.ligature_sets).unwrap() as u16).write_into(writer);
        self.ligature_sets.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("LigatureSubstFormat1")
    }
}

impl Validate for LigatureSubstFormat1 {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("LigatureSubstFormat1", |ctx| {
            ctx.in_field("coverage", |ctx| {
                self.coverage.validate_impl(ctx);
            });
            ctx.in_field("ligature_sets", |ctx| {
                if self.ligature_sets.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
                self.ligature_sets.validate_impl(ctx);
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::tables::gsub::LigatureSubstFormat1<'a>> for LigatureSubstFormat1 {
    fn from_obj_ref(obj: &read_fonts::tables::gsub::LigatureSubstFormat1<'a>, _: FontData) -> Self {
        LigatureSubstFormat1 {
            coverage: obj.coverage().to_owned_table(),
            ligature_sets: obj.ligature_sets().to_owned_table(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::gsub::LigatureSubstFormat1<'a>> for LigatureSubstFormat1 {}

impl<'a> FontRead<'a> for LigatureSubstFormat1 {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::gsub::LigatureSubstFormat1 as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}

/// Part of [LigatureSubstFormat1]
#[derive(Clone, Debug, Default)]
pub struct LigatureSet {
    /// Array of offsets to Ligature tables. Offsets are from beginning
    /// of LigatureSet table, ordered by preference.
    pub ligatures: Vec<OffsetMarker<Ligature>>,
}

impl LigatureSet {
    /// Construct a new `LigatureSet`
    pub fn new(ligatures: Vec<Ligature>) -> Self {
        Self {
            ligatures: ligatures.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for LigatureSet {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (array_len(&self.ligatures).unwrap() as u16).write_into(writer);
        self.ligatures.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("LigatureSet")
    }
}

impl Validate for LigatureSet {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("LigatureSet", |ctx| {
            ctx.in_field("ligatures", |ctx| {
                if self.ligatures.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
                self.ligatures.validate_impl(ctx);
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::tables::gsub::LigatureSet<'a>> for LigatureSet {
    fn from_obj_ref(obj: &read_fonts::tables::gsub::LigatureSet<'a>, _: FontData) -> Self {
        LigatureSet {
            ligatures: obj.ligatures().to_owned_table(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::gsub::LigatureSet<'a>> for LigatureSet {}

impl<'a> FontRead<'a> for LigatureSet {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::gsub::LigatureSet as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

/// Part of [LigatureSubstFormat1]
#[derive(Clone, Debug, Default)]
pub struct Ligature {
    /// glyph ID of ligature to substitute
    pub ligature_glyph: GlyphId,
    /// Array of component glyph IDs — start with the second
    /// component, ordered in writing direction
    pub component_glyph_ids: Vec<GlyphId>,
}

impl Ligature {
    /// Construct a new `Ligature`
    pub fn new(ligature_glyph: GlyphId, component_glyph_ids: Vec<GlyphId>) -> Self {
        Self {
            ligature_glyph,
            component_glyph_ids: component_glyph_ids.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for Ligature {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        self.ligature_glyph.write_into(writer);
        (plus_one(&self.component_glyph_ids.len()).unwrap() as u16).write_into(writer);
        self.component_glyph_ids.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("Ligature")
    }
}

impl Validate for Ligature {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl<'a> FromObjRef<read_fonts::tables::gsub::Ligature<'a>> for Ligature {
    fn from_obj_ref(obj: &read_fonts::tables::gsub::Ligature<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        Ligature {
            ligature_glyph: obj.ligature_glyph(),
            component_glyph_ids: obj.component_glyph_ids().to_owned_obj(offset_data),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::gsub::Ligature<'a>> for Ligature {}

impl<'a> FontRead<'a> for Ligature {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::gsub::Ligature as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

/// [Extension Substitution Subtable Format 1](https://learn.microsoft.com/en-us/typography/opentype/spec/gsub#71-extension-substitution-subtable-format-1)
#[derive(Clone, Debug, Default)]
pub struct ExtensionSubstFormat1<T> {
    /// Lookup type of subtable referenced by extensionOffset (that is,
    /// the extension subtable).
    pub extension_lookup_type: u16,
    /// Offset to the extension subtable, of lookup type
    /// extensionLookupType, relative to the start of the
    /// ExtensionSubstFormat1 subtable.
    pub extension: OffsetMarker<T, WIDTH_32>,
}

impl<T> ExtensionSubstFormat1<T> {
    /// Construct a new `ExtensionSubstFormat1`
    pub fn new(extension_lookup_type: u16, extension: T) -> Self {
        Self {
            extension_lookup_type,
            extension: extension.into(),
        }
    }
}

impl<T: Validate> Validate for ExtensionSubstFormat1<T> {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("ExtensionSubstFormat1", |ctx| {
            ctx.in_field("extension", |ctx| {
                self.extension.validate_impl(ctx);
            });
        })
    }
}

impl<'a, T, U> FromObjRef<read_fonts::tables::gsub::ExtensionSubstFormat1<'a, U>>
    for ExtensionSubstFormat1<T>
where
    U: FontRead<'a>,
    T: FromTableRef<U> + Default + 'static,
{
    fn from_obj_ref(
        obj: &read_fonts::tables::gsub::ExtensionSubstFormat1<'a, U>,
        _: FontData,
    ) -> Self {
        ExtensionSubstFormat1 {
            extension_lookup_type: obj.extension_lookup_type(),
            extension: obj.extension().to_owned_table(),
        }
    }
}

impl<'a, T, U> FromTableRef<read_fonts::tables::gsub::ExtensionSubstFormat1<'a, U>>
    for ExtensionSubstFormat1<T>
where
    U: FontRead<'a>,
    T: FromTableRef<U> + Default + 'static,
{
}

/// A [GSUB Extension Substitution](https://learn.microsoft.com/en-us/typography/opentype/spec/gsub#ES) subtable
#[derive(Debug, Clone)]
pub enum ExtensionSubtable {
    Single(ExtensionSubstFormat1<SingleSubst>),
    Multiple(ExtensionSubstFormat1<MultipleSubstFormat1>),
    Alternate(ExtensionSubstFormat1<AlternateSubstFormat1>),
    Ligature(ExtensionSubstFormat1<LigatureSubstFormat1>),
    Contextual(ExtensionSubstFormat1<SubstitutionSequenceContext>),
    ChainContextual(ExtensionSubstFormat1<SubstitutionChainContext>),
    Reverse(ExtensionSubstFormat1<ReverseChainSingleSubstFormat1>),
}

impl Default for ExtensionSubtable {
    fn default() -> Self {
        Self::Single(Default::default())
    }
}

impl FontWrite for ExtensionSubtable {
    fn write_into(&self, writer: &mut TableWriter) {
        match self {
            Self::Single(table) => table.write_into(writer),
            Self::Multiple(table) => table.write_into(writer),
            Self::Alternate(table) => table.write_into(writer),
            Self::Ligature(table) => table.write_into(writer),
            Self::Contextual(table) => table.write_into(writer),
            Self::ChainContextual(table) => table.write_into(writer),
            Self::Reverse(table) => table.write_into(writer),
        }
    }
    fn table_type(&self) -> TableType {
        match self {
            Self::Single(table) => table.table_type(),
            Self::Multiple(table) => table.table_type(),
            Self::Alternate(table) => table.table_type(),
            Self::Ligature(table) => table.table_type(),
            Self::Contextual(table) => table.table_type(),
            Self::ChainContextual(table) => table.table_type(),
            Self::Reverse(table) => table.table_type(),
        }
    }
}

impl Validate for ExtensionSubtable {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        match self {
            Self::Single(table) => table.validate_impl(ctx),
            Self::Multiple(table) => table.validate_impl(ctx),
            Self::Alternate(table) => table.validate_impl(ctx),
            Self::Ligature(table) => table.validate_impl(ctx),
            Self::Contextual(table) => table.validate_impl(ctx),
            Self::ChainContextual(table) => table.validate_impl(ctx),
            Self::Reverse(table) => table.validate_impl(ctx),
        }
    }
}

impl FromObjRef<read_fonts::tables::gsub::ExtensionSubtable<'_>> for ExtensionSubtable {
    fn from_obj_ref(
        from: &read_fonts::tables::gsub::ExtensionSubtable<'_>,
        data: FontData,
    ) -> Self {
        match from {
            read_fonts::tables::gsub::ExtensionSubtable::Single(table) => {
                Self::Single(table.to_owned_obj(data))
            }
            read_fonts::tables::gsub::ExtensionSubtable::Multiple(table) => {
                Self::Multiple(table.to_owned_obj(data))
            }
            read_fonts::tables::gsub::ExtensionSubtable::Alternate(table) => {
                Self::Alternate(table.to_owned_obj(data))
            }
            read_fonts::tables::gsub::ExtensionSubtable::Ligature(table) => {
                Self::Ligature(table.to_owned_obj(data))
            }
            read_fonts::tables::gsub::ExtensionSubtable::Contextual(table) => {
                Self::Contextual(table.to_owned_obj(data))
            }
            read_fonts::tables::gsub::ExtensionSubtable::ChainContextual(table) => {
                Self::ChainContextual(table.to_owned_obj(data))
            }
            read_fonts::tables::gsub::ExtensionSubtable::Reverse(table) => {
                Self::Reverse(table.to_owned_obj(data))
            }
        }
    }
}

impl FromTableRef<read_fonts::tables::gsub::ExtensionSubtable<'_>> for ExtensionSubtable {}

/// [Reverse Chaining Contextual Single Substitution Format 1](https://learn.microsoft.com/en-us/typography/opentype/spec/gsub#81-reverse-chaining-contextual-single-substitution-format-1-coverage-based-glyph-contexts)
#[derive(Clone, Debug, Default)]
pub struct ReverseChainSingleSubstFormat1 {
    /// Offset to Coverage table, from beginning of substitution
    /// subtable.
    pub coverage: OffsetMarker<CoverageTable>,
    /// Array of offsets to coverage tables in backtrack sequence, in
    /// glyph sequence order.
    pub backtrack_coverages: Vec<OffsetMarker<CoverageTable>>,
    /// Array of offsets to coverage tables in lookahead sequence, in
    /// glyph sequence order.
    pub lookahead_coverages: Vec<OffsetMarker<CoverageTable>>,
    /// Array of substitute glyph IDs — ordered by Coverage index.
    pub substitute_glyph_ids: Vec<GlyphId>,
}

impl ReverseChainSingleSubstFormat1 {
    /// Construct a new `ReverseChainSingleSubstFormat1`
    pub fn new(
        coverage: CoverageTable,
        backtrack_coverages: Vec<CoverageTable>,
        lookahead_coverages: Vec<CoverageTable>,
        substitute_glyph_ids: Vec<GlyphId>,
    ) -> Self {
        Self {
            coverage: coverage.into(),
            backtrack_coverages: backtrack_coverages.into_iter().map(Into::into).collect(),
            lookahead_coverages: lookahead_coverages.into_iter().map(Into::into).collect(),
            substitute_glyph_ids: substitute_glyph_ids.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for ReverseChainSingleSubstFormat1 {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (1 as u16).write_into(writer);
        self.coverage.write_into(writer);
        (array_len(&self.backtrack_coverages).unwrap() as u16).write_into(writer);
        self.backtrack_coverages.write_into(writer);
        (array_len(&self.lookahead_coverages).unwrap() as u16).write_into(writer);
        self.lookahead_coverages.write_into(writer);
        (array_len(&self.substitute_glyph_ids).unwrap() as u16).write_into(writer);
        self.substitute_glyph_ids.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("ReverseChainSingleSubstFormat1")
    }
}

impl Validate for ReverseChainSingleSubstFormat1 {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("ReverseChainSingleSubstFormat1", |ctx| {
            ctx.in_field("coverage", |ctx| {
                self.coverage.validate_impl(ctx);
            });
            ctx.in_field("backtrack_coverages", |ctx| {
                if self.backtrack_coverages.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
                self.backtrack_coverages.validate_impl(ctx);
            });
            ctx.in_field("lookahead_coverages", |ctx| {
                if self.lookahead_coverages.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
                self.lookahead_coverages.validate_impl(ctx);
            });
            ctx.in_field("substitute_glyph_ids", |ctx| {
                if self.substitute_glyph_ids.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::tables::gsub::ReverseChainSingleSubstFormat1<'a>>
    for ReverseChainSingleSubstFormat1
{
    fn from_obj_ref(
        obj: &read_fonts::tables::gsub::ReverseChainSingleSubstFormat1<'a>,
        _: FontData,
    ) -> Self {
        let offset_data = obj.offset_data();
        ReverseChainSingleSubstFormat1 {
            coverage: obj.coverage().to_owned_table(),
            backtrack_coverages: obj.backtrack_coverages().to_owned_table(),
            lookahead_coverages: obj.lookahead_coverages().to_owned_table(),
            substitute_glyph_ids: obj.substitute_glyph_ids().to_owned_obj(offset_data),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::gsub::ReverseChainSingleSubstFormat1<'a>>
    for ReverseChainSingleSubstFormat1
{
}

impl<'a> FontRead<'a> for ReverseChainSingleSubstFormat1 {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::gsub::ReverseChainSingleSubstFormat1 as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}
