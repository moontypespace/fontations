// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

pub use read_fonts::tables::stat::AxisValueTableFlags;

/// [STAT](https://docs.microsoft.com/en-us/typography/opentype/spec/stat) (Style Attributes Table)
#[derive(Clone, Debug, Default)]
pub struct Stat {
    /// Offset in bytes from the beginning of the STAT table to the
    /// start of the design axes array. If designAxisCount is zero, set
    /// to zero; if designAxisCount is greater than zero, must be
    /// greater than zero.
    pub design_axes: OffsetMarker<Vec<AxisRecord>, WIDTH_32>,
    /// Offset in bytes from the beginning of the STAT table to the
    /// start of the design axes value offsets array. If axisValueCount
    /// is zero, set to zero; if axisValueCount is greater than zero,
    /// must be greater than zero.
    pub offset_to_axis_values: OffsetMarker<Vec<OffsetMarker<AxisValue>>, WIDTH_32>,
    /// Name ID used as fallback when projection of names into a
    /// particular font model produces a subfamily name containing only
    /// elidable elements.
    pub elided_fallback_name_id: Option<NameId>,
}

impl FontWrite for Stat {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        let version = MajorMinor::VERSION_1_2 as MajorMinor;
        version.write_into(writer);
        (8 as u16).write_into(writer);
        (array_len(&self.design_axes).unwrap() as u16).write_into(writer);
        self.design_axes.write_into(writer);
        (array_len(&self.offset_to_axis_values).unwrap() as u16).write_into(writer);
        self.offset_to_axis_values.write_into(writer);
        version.compatible((1, 1)).then(|| {
            self.elided_fallback_name_id
                .as_ref()
                .expect("missing versioned field should have failed validation")
                .write_into(writer)
        });
    }
    fn table_type(&self) -> TableType {
        TableType::TopLevel(Stat::TAG)
    }
}

impl Validate for Stat {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("Stat", |ctx| {
            let version: MajorMinor = MajorMinor::VERSION_1_2;
            ctx.in_field("design_axes", |ctx| {
                self.design_axes.validate_impl(ctx);
            });
            ctx.in_field("offset_to_axis_values", |ctx| {
                self.offset_to_axis_values.validate_impl(ctx);
            });
            ctx.in_field("elided_fallback_name_id", |ctx| {
                if version.compatible((1, 1)) && self.elided_fallback_name_id.is_none() {
                    ctx.report(format!("field must be present for version {version}"));
                }
            });
        })
    }
}

impl TopLevelTable for Stat {
    const TAG: Tag = Tag::new(b"STAT");
}

impl<'a> FromObjRef<read_fonts::tables::stat::Stat<'a>> for Stat {
    fn from_obj_ref(obj: &read_fonts::tables::stat::Stat<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        Stat {
            design_axes: obj.design_axes().to_owned_obj(offset_data),
            offset_to_axis_values: convert_axis_value_offsets(obj.offset_to_axis_values()),
            elided_fallback_name_id: obj.elided_fallback_name_id(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::stat::Stat<'a>> for Stat {}

impl<'a> FontRead<'a> for Stat {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::stat::Stat as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

/// [Axis Records](https://docs.microsoft.com/en-us/typography/opentype/spec/stat#axis-records)
#[derive(Clone, Debug, Default)]
pub struct AxisRecord {
    /// A tag identifying the axis of design variation.
    pub axis_tag: Tag,
    /// The name ID for entries in the 'name' table that provide a
    /// display string for this axis.
    pub axis_name_id: NameId,
    /// A value that applications can use to determine primary sorting
    /// of face names, or for ordering of labels when composing family
    /// or face names.
    pub axis_ordering: u16,
}

impl AxisRecord {
    /// Construct a new `AxisRecord`
    pub fn new(axis_tag: Tag, axis_name_id: NameId, axis_ordering: u16) -> Self {
        Self {
            axis_tag,
            axis_name_id,
            axis_ordering,
        }
    }
}

impl FontWrite for AxisRecord {
    fn write_into(&self, writer: &mut TableWriter) {
        self.axis_tag.write_into(writer);
        self.axis_name_id.write_into(writer);
        self.axis_ordering.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("AxisRecord")
    }
}

impl Validate for AxisRecord {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl FromObjRef<read_fonts::tables::stat::AxisRecord> for AxisRecord {
    fn from_obj_ref(obj: &read_fonts::tables::stat::AxisRecord, _: FontData) -> Self {
        AxisRecord {
            axis_tag: obj.axis_tag(),
            axis_name_id: obj.axis_name_id(),
            axis_ordering: obj.axis_ordering(),
        }
    }
}

/// An array of [AxisValue] tables.
#[derive(Clone, Debug, Default)]
pub struct AxisValueArray {
    /// Array of offsets to axis value tables, in bytes from the start
    /// of the axis value offsets array.
    pub axis_values: Vec<OffsetMarker<AxisValue>>,
}

impl AxisValueArray {
    /// Construct a new `AxisValueArray`
    pub fn new(axis_values: Vec<AxisValue>) -> Self {
        Self {
            axis_values: axis_values.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for AxisValueArray {
    fn write_into(&self, writer: &mut TableWriter) {
        self.axis_values.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("AxisValueArray")
    }
}

impl Validate for AxisValueArray {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("AxisValueArray", |ctx| {
            ctx.in_field("axis_values", |ctx| {
                if self.axis_values.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
                self.axis_values.validate_impl(ctx);
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::tables::stat::AxisValueArray<'a>> for AxisValueArray {
    fn from_obj_ref(obj: &read_fonts::tables::stat::AxisValueArray<'a>, _: FontData) -> Self {
        AxisValueArray {
            axis_values: obj.axis_values().to_owned_table(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::stat::AxisValueArray<'a>> for AxisValueArray {}

/// [Axis Value Tables](https://docs.microsoft.com/en-us/typography/opentype/spec/stat#axis-value-tables)
#[derive(Clone, Debug)]
pub enum AxisValue {
    Format1(AxisValueFormat1),
    Format2(AxisValueFormat2),
    Format3(AxisValueFormat3),
    Format4(AxisValueFormat4),
}

impl AxisValue {
    /// Construct a new `AxisValueFormat1` subtable
    pub fn format_1(
        axis_index: u16,
        flags: AxisValueTableFlags,
        value_name_id: NameId,
        value: Fixed,
    ) -> Self {
        Self::Format1(AxisValueFormat1::new(
            axis_index,
            flags,
            value_name_id,
            value,
        ))
    }

    /// Construct a new `AxisValueFormat2` subtable
    pub fn format_2(
        axis_index: u16,
        flags: AxisValueTableFlags,
        value_name_id: NameId,
        nominal_value: Fixed,
        range_min_value: Fixed,
        range_max_value: Fixed,
    ) -> Self {
        Self::Format2(AxisValueFormat2::new(
            axis_index,
            flags,
            value_name_id,
            nominal_value,
            range_min_value,
            range_max_value,
        ))
    }

    /// Construct a new `AxisValueFormat3` subtable
    pub fn format_3(
        axis_index: u16,
        flags: AxisValueTableFlags,
        value_name_id: NameId,
        value: Fixed,
        linked_value: Fixed,
    ) -> Self {
        Self::Format3(AxisValueFormat3::new(
            axis_index,
            flags,
            value_name_id,
            value,
            linked_value,
        ))
    }

    /// Construct a new `AxisValueFormat4` subtable
    pub fn format_4(
        flags: AxisValueTableFlags,
        value_name_id: NameId,
        axis_values: Vec<AxisValueRecord>,
    ) -> Self {
        Self::Format4(AxisValueFormat4::new(flags, value_name_id, axis_values))
    }
}

impl Default for AxisValue {
    fn default() -> Self {
        Self::Format1(Default::default())
    }
}

impl FontWrite for AxisValue {
    fn write_into(&self, writer: &mut TableWriter) {
        match self {
            Self::Format1(item) => item.write_into(writer),
            Self::Format2(item) => item.write_into(writer),
            Self::Format3(item) => item.write_into(writer),
            Self::Format4(item) => item.write_into(writer),
        }
    }
    fn table_type(&self) -> TableType {
        match self {
            Self::Format1(item) => item.table_type(),
            Self::Format2(item) => item.table_type(),
            Self::Format3(item) => item.table_type(),
            Self::Format4(item) => item.table_type(),
        }
    }
}

impl Validate for AxisValue {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        match self {
            Self::Format1(item) => item.validate_impl(ctx),
            Self::Format2(item) => item.validate_impl(ctx),
            Self::Format3(item) => item.validate_impl(ctx),
            Self::Format4(item) => item.validate_impl(ctx),
        }
    }
}

impl FromObjRef<read_fonts::tables::stat::AxisValue<'_>> for AxisValue {
    fn from_obj_ref(obj: &read_fonts::tables::stat::AxisValue, _: FontData) -> Self {
        use read_fonts::tables::stat::AxisValue as ObjRefType;
        match obj {
            ObjRefType::Format1(item) => AxisValue::Format1(item.to_owned_table()),
            ObjRefType::Format2(item) => AxisValue::Format2(item.to_owned_table()),
            ObjRefType::Format3(item) => AxisValue::Format3(item.to_owned_table()),
            ObjRefType::Format4(item) => AxisValue::Format4(item.to_owned_table()),
        }
    }
}

impl FromTableRef<read_fonts::tables::stat::AxisValue<'_>> for AxisValue {}

impl<'a> FontRead<'a> for AxisValue {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::stat::AxisValue as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

/// [Axis value table format 1](https://docs.microsoft.com/en-us/typography/opentype/spec/stat#axis-value-table-format-1)
#[derive(Clone, Debug, Default)]
pub struct AxisValueFormat1 {
    /// Zero-base index into the axis record array identifying the axis
    /// of design variation to which the axis value table applies. Must
    /// be less than designAxisCount.
    pub axis_index: u16,
    /// Flags — see below for details.
    pub flags: AxisValueTableFlags,
    /// The name ID for entries in the 'name' table that provide a
    /// display string for this attribute value.
    pub value_name_id: NameId,
    /// A numeric value for this attribute value.
    pub value: Fixed,
}

impl AxisValueFormat1 {
    /// Construct a new `AxisValueFormat1`
    pub fn new(
        axis_index: u16,
        flags: AxisValueTableFlags,
        value_name_id: NameId,
        value: Fixed,
    ) -> Self {
        Self {
            axis_index,
            flags,
            value_name_id,
            value,
        }
    }
}

impl FontWrite for AxisValueFormat1 {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (1 as u16).write_into(writer);
        self.axis_index.write_into(writer);
        self.flags.write_into(writer);
        self.value_name_id.write_into(writer);
        self.value.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("AxisValueFormat1")
    }
}

impl Validate for AxisValueFormat1 {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl<'a> FromObjRef<read_fonts::tables::stat::AxisValueFormat1<'a>> for AxisValueFormat1 {
    fn from_obj_ref(obj: &read_fonts::tables::stat::AxisValueFormat1<'a>, _: FontData) -> Self {
        AxisValueFormat1 {
            axis_index: obj.axis_index(),
            flags: obj.flags(),
            value_name_id: obj.value_name_id(),
            value: obj.value(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::stat::AxisValueFormat1<'a>> for AxisValueFormat1 {}

impl<'a> FontRead<'a> for AxisValueFormat1 {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::stat::AxisValueFormat1 as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}

/// [Axis value table format 2](https://docs.microsoft.com/en-us/typography/opentype/spec/stat#axis-value-table-format-2)
#[derive(Clone, Debug, Default)]
pub struct AxisValueFormat2 {
    /// Zero-base index into the axis record array identifying the axis
    /// of design variation to which the axis value table applies. Must
    /// be less than designAxisCount.
    pub axis_index: u16,
    /// Flags — see below for details.
    pub flags: AxisValueTableFlags,
    /// The name ID for entries in the 'name' table that provide a
    /// display string for this attribute value.
    pub value_name_id: NameId,
    /// A nominal numeric value for this attribute value.
    pub nominal_value: Fixed,
    /// The minimum value for a range associated with the specified
    /// name ID.
    pub range_min_value: Fixed,
    /// The maximum value for a range associated with the specified
    /// name ID.
    pub range_max_value: Fixed,
}

impl AxisValueFormat2 {
    /// Construct a new `AxisValueFormat2`
    pub fn new(
        axis_index: u16,
        flags: AxisValueTableFlags,
        value_name_id: NameId,
        nominal_value: Fixed,
        range_min_value: Fixed,
        range_max_value: Fixed,
    ) -> Self {
        Self {
            axis_index,
            flags,
            value_name_id,
            nominal_value,
            range_min_value,
            range_max_value,
        }
    }
}

impl FontWrite for AxisValueFormat2 {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (2 as u16).write_into(writer);
        self.axis_index.write_into(writer);
        self.flags.write_into(writer);
        self.value_name_id.write_into(writer);
        self.nominal_value.write_into(writer);
        self.range_min_value.write_into(writer);
        self.range_max_value.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("AxisValueFormat2")
    }
}

impl Validate for AxisValueFormat2 {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl<'a> FromObjRef<read_fonts::tables::stat::AxisValueFormat2<'a>> for AxisValueFormat2 {
    fn from_obj_ref(obj: &read_fonts::tables::stat::AxisValueFormat2<'a>, _: FontData) -> Self {
        AxisValueFormat2 {
            axis_index: obj.axis_index(),
            flags: obj.flags(),
            value_name_id: obj.value_name_id(),
            nominal_value: obj.nominal_value(),
            range_min_value: obj.range_min_value(),
            range_max_value: obj.range_max_value(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::stat::AxisValueFormat2<'a>> for AxisValueFormat2 {}

impl<'a> FontRead<'a> for AxisValueFormat2 {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::stat::AxisValueFormat2 as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}

/// [Axis value table format 3](https://docs.microsoft.com/en-us/typography/opentype/spec/stat#axis-value-table-format-3)
#[derive(Clone, Debug, Default)]
pub struct AxisValueFormat3 {
    /// Zero-base index into the axis record array identifying the axis
    /// of design variation to which the axis value table applies. Must
    /// be less than designAxisCount.
    pub axis_index: u16,
    /// Flags — see below for details.
    pub flags: AxisValueTableFlags,
    /// The name ID for entries in the 'name' table that provide a
    /// display string for this attribute value.
    pub value_name_id: NameId,
    /// A numeric value for this attribute value.
    pub value: Fixed,
    /// The numeric value for a style-linked mapping from this value.
    pub linked_value: Fixed,
}

impl AxisValueFormat3 {
    /// Construct a new `AxisValueFormat3`
    pub fn new(
        axis_index: u16,
        flags: AxisValueTableFlags,
        value_name_id: NameId,
        value: Fixed,
        linked_value: Fixed,
    ) -> Self {
        Self {
            axis_index,
            flags,
            value_name_id,
            value,
            linked_value,
        }
    }
}

impl FontWrite for AxisValueFormat3 {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (3 as u16).write_into(writer);
        self.axis_index.write_into(writer);
        self.flags.write_into(writer);
        self.value_name_id.write_into(writer);
        self.value.write_into(writer);
        self.linked_value.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("AxisValueFormat3")
    }
}

impl Validate for AxisValueFormat3 {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl<'a> FromObjRef<read_fonts::tables::stat::AxisValueFormat3<'a>> for AxisValueFormat3 {
    fn from_obj_ref(obj: &read_fonts::tables::stat::AxisValueFormat3<'a>, _: FontData) -> Self {
        AxisValueFormat3 {
            axis_index: obj.axis_index(),
            flags: obj.flags(),
            value_name_id: obj.value_name_id(),
            value: obj.value(),
            linked_value: obj.linked_value(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::stat::AxisValueFormat3<'a>> for AxisValueFormat3 {}

impl<'a> FontRead<'a> for AxisValueFormat3 {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::stat::AxisValueFormat3 as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}

/// [Axis value table format 4](https://docs.microsoft.com/en-us/typography/opentype/spec/stat#axis-value-table-format-4)
#[derive(Clone, Debug, Default)]
pub struct AxisValueFormat4 {
    /// Flags — see below for details.
    pub flags: AxisValueTableFlags,
    /// The name ID for entries in the 'name' table that provide a
    /// display string for this combination of axis values.
    pub value_name_id: NameId,
    /// Array of AxisValue records that provide the combination of axis
    /// values, one for each contributing axis.
    pub axis_values: Vec<AxisValueRecord>,
}

impl AxisValueFormat4 {
    /// Construct a new `AxisValueFormat4`
    pub fn new(
        flags: AxisValueTableFlags,
        value_name_id: NameId,
        axis_values: Vec<AxisValueRecord>,
    ) -> Self {
        Self {
            flags,
            value_name_id,
            axis_values: axis_values.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for AxisValueFormat4 {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (4 as u16).write_into(writer);
        (array_len(&self.axis_values).unwrap() as u16).write_into(writer);
        self.flags.write_into(writer);
        self.value_name_id.write_into(writer);
        self.axis_values.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("AxisValueFormat4")
    }
}

impl Validate for AxisValueFormat4 {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("AxisValueFormat4", |ctx| {
            ctx.in_field("axis_values", |ctx| {
                if self.axis_values.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
                self.axis_values.validate_impl(ctx);
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::tables::stat::AxisValueFormat4<'a>> for AxisValueFormat4 {
    fn from_obj_ref(obj: &read_fonts::tables::stat::AxisValueFormat4<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        AxisValueFormat4 {
            flags: obj.flags(),
            value_name_id: obj.value_name_id(),
            axis_values: obj.axis_values().to_owned_obj(offset_data),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::stat::AxisValueFormat4<'a>> for AxisValueFormat4 {}

impl<'a> FontRead<'a> for AxisValueFormat4 {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::stat::AxisValueFormat4 as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}

/// Part of [AxisValueFormat4]
#[derive(Clone, Debug, Default)]
pub struct AxisValueRecord {
    /// Zero-base index into the axis record array identifying the axis
    /// to which this value applies. Must be less than designAxisCount.
    pub axis_index: u16,
    /// A numeric value for this attribute value.
    pub value: Fixed,
}

impl AxisValueRecord {
    /// Construct a new `AxisValueRecord`
    pub fn new(axis_index: u16, value: Fixed) -> Self {
        Self { axis_index, value }
    }
}

impl FontWrite for AxisValueRecord {
    fn write_into(&self, writer: &mut TableWriter) {
        self.axis_index.write_into(writer);
        self.value.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("AxisValueRecord")
    }
}

impl Validate for AxisValueRecord {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl FromObjRef<read_fonts::tables::stat::AxisValueRecord> for AxisValueRecord {
    fn from_obj_ref(obj: &read_fonts::tables::stat::AxisValueRecord, _: FontData) -> Self {
        AxisValueRecord {
            axis_index: obj.axis_index(),
            value: obj.value(),
        }
    }
}

impl FontWrite for AxisValueTableFlags {
    fn write_into(&self, writer: &mut TableWriter) {
        writer.write_slice(&self.bits().to_be_bytes())
    }
}
