// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702



use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct OpDef {
    // message fields
    pub name: ::std::string::String,
    pub input_arg: ::protobuf::RepeatedField<OpDef_ArgDef>,
    pub output_arg: ::protobuf::RepeatedField<OpDef_ArgDef>,
    pub attr: ::protobuf::RepeatedField<OpDef_AttrDef>,
    pub deprecation: ::protobuf::SingularPtrField<OpDeprecation>,
    pub summary: ::std::string::String,
    pub description: ::std::string::String,
    pub is_commutative: bool,
    pub is_aggregate: bool,
    pub is_stateful: bool,
    pub allows_uninitialized_input: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OpDef {}

impl OpDef {
    pub fn new() -> OpDef {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpDef {
        static mut instance: ::protobuf::lazy::Lazy<OpDef> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpDef,
        };
        unsafe {
            instance.get(OpDef::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // repeated .tensorflow.OpDef.ArgDef input_arg = 2;

    pub fn clear_input_arg(&mut self) {
        self.input_arg.clear();
    }

    // Param is passed by value, moved
    pub fn set_input_arg(&mut self, v: ::protobuf::RepeatedField<OpDef_ArgDef>) {
        self.input_arg = v;
    }

    // Mutable pointer to the field.
    pub fn mut_input_arg(&mut self) -> &mut ::protobuf::RepeatedField<OpDef_ArgDef> {
        &mut self.input_arg
    }

    // Take field
    pub fn take_input_arg(&mut self) -> ::protobuf::RepeatedField<OpDef_ArgDef> {
        ::std::mem::replace(&mut self.input_arg, ::protobuf::RepeatedField::new())
    }

    pub fn get_input_arg(&self) -> &[OpDef_ArgDef] {
        &self.input_arg
    }

    fn get_input_arg_for_reflect(&self) -> &::protobuf::RepeatedField<OpDef_ArgDef> {
        &self.input_arg
    }

    fn mut_input_arg_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<OpDef_ArgDef> {
        &mut self.input_arg
    }

    // repeated .tensorflow.OpDef.ArgDef output_arg = 3;

    pub fn clear_output_arg(&mut self) {
        self.output_arg.clear();
    }

    // Param is passed by value, moved
    pub fn set_output_arg(&mut self, v: ::protobuf::RepeatedField<OpDef_ArgDef>) {
        self.output_arg = v;
    }

    // Mutable pointer to the field.
    pub fn mut_output_arg(&mut self) -> &mut ::protobuf::RepeatedField<OpDef_ArgDef> {
        &mut self.output_arg
    }

    // Take field
    pub fn take_output_arg(&mut self) -> ::protobuf::RepeatedField<OpDef_ArgDef> {
        ::std::mem::replace(&mut self.output_arg, ::protobuf::RepeatedField::new())
    }

    pub fn get_output_arg(&self) -> &[OpDef_ArgDef] {
        &self.output_arg
    }

    fn get_output_arg_for_reflect(&self) -> &::protobuf::RepeatedField<OpDef_ArgDef> {
        &self.output_arg
    }

    fn mut_output_arg_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<OpDef_ArgDef> {
        &mut self.output_arg
    }

    // repeated .tensorflow.OpDef.AttrDef attr = 4;

    pub fn clear_attr(&mut self) {
        self.attr.clear();
    }

    // Param is passed by value, moved
    pub fn set_attr(&mut self, v: ::protobuf::RepeatedField<OpDef_AttrDef>) {
        self.attr = v;
    }

    // Mutable pointer to the field.
    pub fn mut_attr(&mut self) -> &mut ::protobuf::RepeatedField<OpDef_AttrDef> {
        &mut self.attr
    }

    // Take field
    pub fn take_attr(&mut self) -> ::protobuf::RepeatedField<OpDef_AttrDef> {
        ::std::mem::replace(&mut self.attr, ::protobuf::RepeatedField::new())
    }

    pub fn get_attr(&self) -> &[OpDef_AttrDef] {
        &self.attr
    }

    fn get_attr_for_reflect(&self) -> &::protobuf::RepeatedField<OpDef_AttrDef> {
        &self.attr
    }

    fn mut_attr_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<OpDef_AttrDef> {
        &mut self.attr
    }

    // .tensorflow.OpDeprecation deprecation = 8;

    pub fn clear_deprecation(&mut self) {
        self.deprecation.clear();
    }

    pub fn has_deprecation(&self) -> bool {
        self.deprecation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deprecation(&mut self, v: OpDeprecation) {
        self.deprecation = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_deprecation(&mut self) -> &mut OpDeprecation {
        if self.deprecation.is_none() {
            self.deprecation.set_default();
        }
        self.deprecation.as_mut().unwrap()
    }

    // Take field
    pub fn take_deprecation(&mut self) -> OpDeprecation {
        self.deprecation.take().unwrap_or_else(|| OpDeprecation::new())
    }

    pub fn get_deprecation(&self) -> &OpDeprecation {
        self.deprecation.as_ref().unwrap_or_else(|| OpDeprecation::default_instance())
    }

    fn get_deprecation_for_reflect(&self) -> &::protobuf::SingularPtrField<OpDeprecation> {
        &self.deprecation
    }

    fn mut_deprecation_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<OpDeprecation> {
        &mut self.deprecation
    }

    // string summary = 5;

    pub fn clear_summary(&mut self) {
        self.summary.clear();
    }

    // Param is passed by value, moved
    pub fn set_summary(&mut self, v: ::std::string::String) {
        self.summary = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_summary(&mut self) -> &mut ::std::string::String {
        &mut self.summary
    }

    // Take field
    pub fn take_summary(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.summary, ::std::string::String::new())
    }

    pub fn get_summary(&self) -> &str {
        &self.summary
    }

    fn get_summary_for_reflect(&self) -> &::std::string::String {
        &self.summary
    }

    fn mut_summary_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.summary
    }

    // string description = 6;

    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.description, ::std::string::String::new())
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    fn get_description_for_reflect(&self) -> &::std::string::String {
        &self.description
    }

    fn mut_description_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }

    // bool is_commutative = 18;

    pub fn clear_is_commutative(&mut self) {
        self.is_commutative = false;
    }

    // Param is passed by value, moved
    pub fn set_is_commutative(&mut self, v: bool) {
        self.is_commutative = v;
    }

    pub fn get_is_commutative(&self) -> bool {
        self.is_commutative
    }

    fn get_is_commutative_for_reflect(&self) -> &bool {
        &self.is_commutative
    }

    fn mut_is_commutative_for_reflect(&mut self) -> &mut bool {
        &mut self.is_commutative
    }

    // bool is_aggregate = 16;

    pub fn clear_is_aggregate(&mut self) {
        self.is_aggregate = false;
    }

    // Param is passed by value, moved
    pub fn set_is_aggregate(&mut self, v: bool) {
        self.is_aggregate = v;
    }

    pub fn get_is_aggregate(&self) -> bool {
        self.is_aggregate
    }

    fn get_is_aggregate_for_reflect(&self) -> &bool {
        &self.is_aggregate
    }

    fn mut_is_aggregate_for_reflect(&mut self) -> &mut bool {
        &mut self.is_aggregate
    }

    // bool is_stateful = 17;

    pub fn clear_is_stateful(&mut self) {
        self.is_stateful = false;
    }

    // Param is passed by value, moved
    pub fn set_is_stateful(&mut self, v: bool) {
        self.is_stateful = v;
    }

    pub fn get_is_stateful(&self) -> bool {
        self.is_stateful
    }

    fn get_is_stateful_for_reflect(&self) -> &bool {
        &self.is_stateful
    }

    fn mut_is_stateful_for_reflect(&mut self) -> &mut bool {
        &mut self.is_stateful
    }

    // bool allows_uninitialized_input = 19;

    pub fn clear_allows_uninitialized_input(&mut self) {
        self.allows_uninitialized_input = false;
    }

    // Param is passed by value, moved
    pub fn set_allows_uninitialized_input(&mut self, v: bool) {
        self.allows_uninitialized_input = v;
    }

    pub fn get_allows_uninitialized_input(&self) -> bool {
        self.allows_uninitialized_input
    }

    fn get_allows_uninitialized_input_for_reflect(&self) -> &bool {
        &self.allows_uninitialized_input
    }

    fn mut_allows_uninitialized_input_for_reflect(&mut self) -> &mut bool {
        &mut self.allows_uninitialized_input
    }
}

impl ::protobuf::Message for OpDef {
    fn is_initialized(&self) -> bool {
        for v in &self.input_arg {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.output_arg {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.attr {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.deprecation {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.input_arg)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.output_arg)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.attr)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.deprecation)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.summary)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.description)?;
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_commutative = tmp;
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_aggregate = tmp;
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_stateful = tmp;
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.allows_uninitialized_input = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        for value in &self.input_arg {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.output_arg {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.attr {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.deprecation.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.summary.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.summary);
        }
        if !self.description.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.description);
        }
        if self.is_commutative != false {
            my_size += 3;
        }
        if self.is_aggregate != false {
            my_size += 3;
        }
        if self.is_stateful != false {
            my_size += 3;
        }
        if self.allows_uninitialized_input != false {
            my_size += 3;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        for v in &self.input_arg {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.output_arg {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.attr {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.deprecation.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.summary.is_empty() {
            os.write_string(5, &self.summary)?;
        }
        if !self.description.is_empty() {
            os.write_string(6, &self.description)?;
        }
        if self.is_commutative != false {
            os.write_bool(18, self.is_commutative)?;
        }
        if self.is_aggregate != false {
            os.write_bool(16, self.is_aggregate)?;
        }
        if self.is_stateful != false {
            os.write_bool(17, self.is_stateful)?;
        }
        if self.allows_uninitialized_input != false {
            os.write_bool(19, self.allows_uninitialized_input)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OpDef {
    fn new() -> OpDef {
        OpDef::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpDef>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    OpDef::get_name_for_reflect,
                    OpDef::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OpDef_ArgDef>>(
                    "input_arg",
                    OpDef::get_input_arg_for_reflect,
                    OpDef::mut_input_arg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OpDef_ArgDef>>(
                    "output_arg",
                    OpDef::get_output_arg_for_reflect,
                    OpDef::mut_output_arg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OpDef_AttrDef>>(
                    "attr",
                    OpDef::get_attr_for_reflect,
                    OpDef::mut_attr_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OpDeprecation>>(
                    "deprecation",
                    OpDef::get_deprecation_for_reflect,
                    OpDef::mut_deprecation_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "summary",
                    OpDef::get_summary_for_reflect,
                    OpDef::mut_summary_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "description",
                    OpDef::get_description_for_reflect,
                    OpDef::mut_description_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_commutative",
                    OpDef::get_is_commutative_for_reflect,
                    OpDef::mut_is_commutative_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_aggregate",
                    OpDef::get_is_aggregate_for_reflect,
                    OpDef::mut_is_aggregate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_stateful",
                    OpDef::get_is_stateful_for_reflect,
                    OpDef::mut_is_stateful_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "allows_uninitialized_input",
                    OpDef::get_allows_uninitialized_input_for_reflect,
                    OpDef::mut_allows_uninitialized_input_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpDef>(
                    "OpDef",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpDef {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_input_arg();
        self.clear_output_arg();
        self.clear_attr();
        self.clear_deprecation();
        self.clear_summary();
        self.clear_description();
        self.clear_is_commutative();
        self.clear_is_aggregate();
        self.clear_is_stateful();
        self.clear_allows_uninitialized_input();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OpDef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OpDef {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OpDef_ArgDef {
    // message fields
    pub name: ::std::string::String,
    pub description: ::std::string::String,
    pub field_type: super::types::DataType,
    pub type_attr: ::std::string::String,
    pub number_attr: ::std::string::String,
    pub type_list_attr: ::std::string::String,
    pub is_ref: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OpDef_ArgDef {}

impl OpDef_ArgDef {
    pub fn new() -> OpDef_ArgDef {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpDef_ArgDef {
        static mut instance: ::protobuf::lazy::Lazy<OpDef_ArgDef> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpDef_ArgDef,
        };
        unsafe {
            instance.get(OpDef_ArgDef::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // string description = 2;

    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.description, ::std::string::String::new())
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    fn get_description_for_reflect(&self) -> &::std::string::String {
        &self.description
    }

    fn mut_description_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }

    // .tensorflow.DataType type = 3;

    pub fn clear_field_type(&mut self) {
        self.field_type = super::types::DataType::DT_INVALID;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: super::types::DataType) {
        self.field_type = v;
    }

    pub fn get_field_type(&self) -> super::types::DataType {
        self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &super::types::DataType {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut super::types::DataType {
        &mut self.field_type
    }

    // string type_attr = 4;

    pub fn clear_type_attr(&mut self) {
        self.type_attr.clear();
    }

    // Param is passed by value, moved
    pub fn set_type_attr(&mut self, v: ::std::string::String) {
        self.type_attr = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_type_attr(&mut self) -> &mut ::std::string::String {
        &mut self.type_attr
    }

    // Take field
    pub fn take_type_attr(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.type_attr, ::std::string::String::new())
    }

    pub fn get_type_attr(&self) -> &str {
        &self.type_attr
    }

    fn get_type_attr_for_reflect(&self) -> &::std::string::String {
        &self.type_attr
    }

    fn mut_type_attr_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.type_attr
    }

    // string number_attr = 5;

    pub fn clear_number_attr(&mut self) {
        self.number_attr.clear();
    }

    // Param is passed by value, moved
    pub fn set_number_attr(&mut self, v: ::std::string::String) {
        self.number_attr = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_number_attr(&mut self) -> &mut ::std::string::String {
        &mut self.number_attr
    }

    // Take field
    pub fn take_number_attr(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.number_attr, ::std::string::String::new())
    }

    pub fn get_number_attr(&self) -> &str {
        &self.number_attr
    }

    fn get_number_attr_for_reflect(&self) -> &::std::string::String {
        &self.number_attr
    }

    fn mut_number_attr_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.number_attr
    }

    // string type_list_attr = 6;

    pub fn clear_type_list_attr(&mut self) {
        self.type_list_attr.clear();
    }

    // Param is passed by value, moved
    pub fn set_type_list_attr(&mut self, v: ::std::string::String) {
        self.type_list_attr = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_type_list_attr(&mut self) -> &mut ::std::string::String {
        &mut self.type_list_attr
    }

    // Take field
    pub fn take_type_list_attr(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.type_list_attr, ::std::string::String::new())
    }

    pub fn get_type_list_attr(&self) -> &str {
        &self.type_list_attr
    }

    fn get_type_list_attr_for_reflect(&self) -> &::std::string::String {
        &self.type_list_attr
    }

    fn mut_type_list_attr_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.type_list_attr
    }

    // bool is_ref = 16;

    pub fn clear_is_ref(&mut self) {
        self.is_ref = false;
    }

    // Param is passed by value, moved
    pub fn set_is_ref(&mut self, v: bool) {
        self.is_ref = v;
    }

    pub fn get_is_ref(&self) -> bool {
        self.is_ref
    }

    fn get_is_ref_for_reflect(&self) -> &bool {
        &self.is_ref
    }

    fn mut_is_ref_for_reflect(&mut self) -> &mut bool {
        &mut self.is_ref
    }
}

impl ::protobuf::Message for OpDef_ArgDef {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.description)?;
                },
                3 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.field_type, 3, &mut self.unknown_fields)?
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.type_attr)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.number_attr)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.type_list_attr)?;
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_ref = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if !self.description.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.description);
        }
        if self.field_type != super::types::DataType::DT_INVALID {
            my_size += ::protobuf::rt::enum_size(3, self.field_type);
        }
        if !self.type_attr.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.type_attr);
        }
        if !self.number_attr.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.number_attr);
        }
        if !self.type_list_attr.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.type_list_attr);
        }
        if self.is_ref != false {
            my_size += 3;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if !self.description.is_empty() {
            os.write_string(2, &self.description)?;
        }
        if self.field_type != super::types::DataType::DT_INVALID {
            os.write_enum(3, self.field_type.value())?;
        }
        if !self.type_attr.is_empty() {
            os.write_string(4, &self.type_attr)?;
        }
        if !self.number_attr.is_empty() {
            os.write_string(5, &self.number_attr)?;
        }
        if !self.type_list_attr.is_empty() {
            os.write_string(6, &self.type_list_attr)?;
        }
        if self.is_ref != false {
            os.write_bool(16, self.is_ref)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OpDef_ArgDef {
    fn new() -> OpDef_ArgDef {
        OpDef_ArgDef::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpDef_ArgDef>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    OpDef_ArgDef::get_name_for_reflect,
                    OpDef_ArgDef::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "description",
                    OpDef_ArgDef::get_description_for_reflect,
                    OpDef_ArgDef::mut_description_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::types::DataType>>(
                    "type",
                    OpDef_ArgDef::get_field_type_for_reflect,
                    OpDef_ArgDef::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type_attr",
                    OpDef_ArgDef::get_type_attr_for_reflect,
                    OpDef_ArgDef::mut_type_attr_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "number_attr",
                    OpDef_ArgDef::get_number_attr_for_reflect,
                    OpDef_ArgDef::mut_number_attr_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type_list_attr",
                    OpDef_ArgDef::get_type_list_attr_for_reflect,
                    OpDef_ArgDef::mut_type_list_attr_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_ref",
                    OpDef_ArgDef::get_is_ref_for_reflect,
                    OpDef_ArgDef::mut_is_ref_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpDef_ArgDef>(
                    "OpDef_ArgDef",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpDef_ArgDef {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_description();
        self.clear_field_type();
        self.clear_type_attr();
        self.clear_number_attr();
        self.clear_type_list_attr();
        self.clear_is_ref();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OpDef_ArgDef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OpDef_ArgDef {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OpDef_AttrDef {
    // message fields
    pub name: ::std::string::String,
    pub field_type: ::std::string::String,
    pub default_value: ::protobuf::SingularPtrField<super::attr_value::AttrValue>,
    pub description: ::std::string::String,
    pub has_minimum: bool,
    pub minimum: i64,
    pub allowed_values: ::protobuf::SingularPtrField<super::attr_value::AttrValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OpDef_AttrDef {}

impl OpDef_AttrDef {
    pub fn new() -> OpDef_AttrDef {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpDef_AttrDef {
        static mut instance: ::protobuf::lazy::Lazy<OpDef_AttrDef> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpDef_AttrDef,
        };
        unsafe {
            instance.get(OpDef_AttrDef::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // string type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::string::String) {
        self.field_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::string::String {
        &mut self.field_type
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.field_type, ::std::string::String::new())
    }

    pub fn get_field_type(&self) -> &str {
        &self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &::std::string::String {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.field_type
    }

    // .tensorflow.AttrValue default_value = 3;

    pub fn clear_default_value(&mut self) {
        self.default_value.clear();
    }

    pub fn has_default_value(&self) -> bool {
        self.default_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_default_value(&mut self, v: super::attr_value::AttrValue) {
        self.default_value = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_default_value(&mut self) -> &mut super::attr_value::AttrValue {
        if self.default_value.is_none() {
            self.default_value.set_default();
        }
        self.default_value.as_mut().unwrap()
    }

    // Take field
    pub fn take_default_value(&mut self) -> super::attr_value::AttrValue {
        self.default_value.take().unwrap_or_else(|| super::attr_value::AttrValue::new())
    }

    pub fn get_default_value(&self) -> &super::attr_value::AttrValue {
        self.default_value.as_ref().unwrap_or_else(|| super::attr_value::AttrValue::default_instance())
    }

    fn get_default_value_for_reflect(&self) -> &::protobuf::SingularPtrField<super::attr_value::AttrValue> {
        &self.default_value
    }

    fn mut_default_value_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::attr_value::AttrValue> {
        &mut self.default_value
    }

    // string description = 4;

    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.description, ::std::string::String::new())
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    fn get_description_for_reflect(&self) -> &::std::string::String {
        &self.description
    }

    fn mut_description_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }

    // bool has_minimum = 5;

    pub fn clear_has_minimum(&mut self) {
        self.has_minimum = false;
    }

    // Param is passed by value, moved
    pub fn set_has_minimum(&mut self, v: bool) {
        self.has_minimum = v;
    }

    pub fn get_has_minimum(&self) -> bool {
        self.has_minimum
    }

    fn get_has_minimum_for_reflect(&self) -> &bool {
        &self.has_minimum
    }

    fn mut_has_minimum_for_reflect(&mut self) -> &mut bool {
        &mut self.has_minimum
    }

    // int64 minimum = 6;

    pub fn clear_minimum(&mut self) {
        self.minimum = 0;
    }

    // Param is passed by value, moved
    pub fn set_minimum(&mut self, v: i64) {
        self.minimum = v;
    }

    pub fn get_minimum(&self) -> i64 {
        self.minimum
    }

    fn get_minimum_for_reflect(&self) -> &i64 {
        &self.minimum
    }

    fn mut_minimum_for_reflect(&mut self) -> &mut i64 {
        &mut self.minimum
    }

    // .tensorflow.AttrValue allowed_values = 7;

    pub fn clear_allowed_values(&mut self) {
        self.allowed_values.clear();
    }

    pub fn has_allowed_values(&self) -> bool {
        self.allowed_values.is_some()
    }

    // Param is passed by value, moved
    pub fn set_allowed_values(&mut self, v: super::attr_value::AttrValue) {
        self.allowed_values = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_allowed_values(&mut self) -> &mut super::attr_value::AttrValue {
        if self.allowed_values.is_none() {
            self.allowed_values.set_default();
        }
        self.allowed_values.as_mut().unwrap()
    }

    // Take field
    pub fn take_allowed_values(&mut self) -> super::attr_value::AttrValue {
        self.allowed_values.take().unwrap_or_else(|| super::attr_value::AttrValue::new())
    }

    pub fn get_allowed_values(&self) -> &super::attr_value::AttrValue {
        self.allowed_values.as_ref().unwrap_or_else(|| super::attr_value::AttrValue::default_instance())
    }

    fn get_allowed_values_for_reflect(&self) -> &::protobuf::SingularPtrField<super::attr_value::AttrValue> {
        &self.allowed_values
    }

    fn mut_allowed_values_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::attr_value::AttrValue> {
        &mut self.allowed_values
    }
}

impl ::protobuf::Message for OpDef_AttrDef {
    fn is_initialized(&self) -> bool {
        for v in &self.default_value {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.allowed_values {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.field_type)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.default_value)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.description)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.has_minimum = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.minimum = tmp;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.allowed_values)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if !self.field_type.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.field_type);
        }
        if let Some(ref v) = self.default_value.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.description.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.description);
        }
        if self.has_minimum != false {
            my_size += 2;
        }
        if self.minimum != 0 {
            my_size += ::protobuf::rt::value_size(6, self.minimum, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.allowed_values.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if !self.field_type.is_empty() {
            os.write_string(2, &self.field_type)?;
        }
        if let Some(ref v) = self.default_value.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.description.is_empty() {
            os.write_string(4, &self.description)?;
        }
        if self.has_minimum != false {
            os.write_bool(5, self.has_minimum)?;
        }
        if self.minimum != 0 {
            os.write_int64(6, self.minimum)?;
        }
        if let Some(ref v) = self.allowed_values.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OpDef_AttrDef {
    fn new() -> OpDef_AttrDef {
        OpDef_AttrDef::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpDef_AttrDef>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    OpDef_AttrDef::get_name_for_reflect,
                    OpDef_AttrDef::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type",
                    OpDef_AttrDef::get_field_type_for_reflect,
                    OpDef_AttrDef::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::attr_value::AttrValue>>(
                    "default_value",
                    OpDef_AttrDef::get_default_value_for_reflect,
                    OpDef_AttrDef::mut_default_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "description",
                    OpDef_AttrDef::get_description_for_reflect,
                    OpDef_AttrDef::mut_description_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "has_minimum",
                    OpDef_AttrDef::get_has_minimum_for_reflect,
                    OpDef_AttrDef::mut_has_minimum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "minimum",
                    OpDef_AttrDef::get_minimum_for_reflect,
                    OpDef_AttrDef::mut_minimum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::attr_value::AttrValue>>(
                    "allowed_values",
                    OpDef_AttrDef::get_allowed_values_for_reflect,
                    OpDef_AttrDef::mut_allowed_values_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpDef_AttrDef>(
                    "OpDef_AttrDef",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpDef_AttrDef {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_field_type();
        self.clear_default_value();
        self.clear_description();
        self.clear_has_minimum();
        self.clear_minimum();
        self.clear_allowed_values();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OpDef_AttrDef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OpDef_AttrDef {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OpDeprecation {
    // message fields
    pub version: i32,
    pub explanation: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OpDeprecation {}

impl OpDeprecation {
    pub fn new() -> OpDeprecation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpDeprecation {
        static mut instance: ::protobuf::lazy::Lazy<OpDeprecation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpDeprecation,
        };
        unsafe {
            instance.get(OpDeprecation::new)
        }
    }

    // int32 version = 1;

    pub fn clear_version(&mut self) {
        self.version = 0;
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: i32) {
        self.version = v;
    }

    pub fn get_version(&self) -> i32 {
        self.version
    }

    fn get_version_for_reflect(&self) -> &i32 {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut i32 {
        &mut self.version
    }

    // string explanation = 2;

    pub fn clear_explanation(&mut self) {
        self.explanation.clear();
    }

    // Param is passed by value, moved
    pub fn set_explanation(&mut self, v: ::std::string::String) {
        self.explanation = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_explanation(&mut self) -> &mut ::std::string::String {
        &mut self.explanation
    }

    // Take field
    pub fn take_explanation(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.explanation, ::std::string::String::new())
    }

    pub fn get_explanation(&self) -> &str {
        &self.explanation
    }

    fn get_explanation_for_reflect(&self) -> &::std::string::String {
        &self.explanation
    }

    fn mut_explanation_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.explanation
    }
}

impl ::protobuf::Message for OpDeprecation {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.version = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.explanation)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.version != 0 {
            my_size += ::protobuf::rt::value_size(1, self.version, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.explanation.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.explanation);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.version != 0 {
            os.write_int32(1, self.version)?;
        }
        if !self.explanation.is_empty() {
            os.write_string(2, &self.explanation)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OpDeprecation {
    fn new() -> OpDeprecation {
        OpDeprecation::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpDeprecation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "version",
                    OpDeprecation::get_version_for_reflect,
                    OpDeprecation::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "explanation",
                    OpDeprecation::get_explanation_for_reflect,
                    OpDeprecation::mut_explanation_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpDeprecation>(
                    "OpDeprecation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpDeprecation {
    fn clear(&mut self) {
        self.clear_version();
        self.clear_explanation();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OpDeprecation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OpDeprecation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OpList {
    // message fields
    pub op: ::protobuf::RepeatedField<OpDef>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OpList {}

impl OpList {
    pub fn new() -> OpList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpList {
        static mut instance: ::protobuf::lazy::Lazy<OpList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpList,
        };
        unsafe {
            instance.get(OpList::new)
        }
    }

    // repeated .tensorflow.OpDef op = 1;

    pub fn clear_op(&mut self) {
        self.op.clear();
    }

    // Param is passed by value, moved
    pub fn set_op(&mut self, v: ::protobuf::RepeatedField<OpDef>) {
        self.op = v;
    }

    // Mutable pointer to the field.
    pub fn mut_op(&mut self) -> &mut ::protobuf::RepeatedField<OpDef> {
        &mut self.op
    }

    // Take field
    pub fn take_op(&mut self) -> ::protobuf::RepeatedField<OpDef> {
        ::std::mem::replace(&mut self.op, ::protobuf::RepeatedField::new())
    }

    pub fn get_op(&self) -> &[OpDef] {
        &self.op
    }

    fn get_op_for_reflect(&self) -> &::protobuf::RepeatedField<OpDef> {
        &self.op
    }

    fn mut_op_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<OpDef> {
        &mut self.op
    }
}

impl ::protobuf::Message for OpList {
    fn is_initialized(&self) -> bool {
        for v in &self.op {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.op)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.op {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.op {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OpList {
    fn new() -> OpList {
        OpList::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OpDef>>(
                    "op",
                    OpList::get_op_for_reflect,
                    OpList::mut_op_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpList>(
                    "OpList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpList {
    fn clear(&mut self) {
        self.clear_op();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OpList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OpList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n&tensorflow/core/framework/op_def.proto\x12\ntensorflow\x1a*tensorflow\
    /core/framework/attr_value.proto\x1a%tensorflow/core/framework/types.pro\
    to\"\xcd\x07\n\x05OpDef\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\
    \x125\n\tinput_arg\x18\x02\x20\x03(\x0b2\x18.tensorflow.OpDef.ArgDefR\
    \x08inputArg\x127\n\noutput_arg\x18\x03\x20\x03(\x0b2\x18.tensorflow.OpD\
    ef.ArgDefR\toutputArg\x12-\n\x04attr\x18\x04\x20\x03(\x0b2\x19.tensorflo\
    w.OpDef.AttrDefR\x04attr\x12;\n\x0bdeprecation\x18\x08\x20\x01(\x0b2\x19\
    .tensorflow.OpDeprecationR\x0bdeprecation\x12\x18\n\x07summary\x18\x05\
    \x20\x01(\tR\x07summary\x12\x20\n\x0bdescription\x18\x06\x20\x01(\tR\x0b\
    description\x12%\n\x0eis_commutative\x18\x12\x20\x01(\x08R\risCommutativ\
    e\x12!\n\x0cis_aggregate\x18\x10\x20\x01(\x08R\x0bisAggregate\x12\x1f\n\
    \x0bis_stateful\x18\x11\x20\x01(\x08R\nisStateful\x12<\n\x1aallows_unini\
    tialized_input\x18\x13\x20\x01(\x08R\x18allowsUninitializedInput\x1a\xe3\
    \x01\n\x06ArgDef\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\x20\n\
    \x0bdescription\x18\x02\x20\x01(\tR\x0bdescription\x12(\n\x04type\x18\
    \x03\x20\x01(\x0e2\x14.tensorflow.DataTypeR\x04type\x12\x1b\n\ttype_attr\
    \x18\x04\x20\x01(\tR\x08typeAttr\x12\x1f\n\x0bnumber_attr\x18\x05\x20\
    \x01(\tR\nnumberAttr\x12$\n\x0etype_list_attr\x18\x06\x20\x01(\tR\x0ctyp\
    eListAttr\x12\x15\n\x06is_ref\x18\x10\x20\x01(\x08R\x05isRef\x1a\x88\x02\
    \n\x07AttrDef\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\x12\n\
    \x04type\x18\x02\x20\x01(\tR\x04type\x12:\n\rdefault_value\x18\x03\x20\
    \x01(\x0b2\x15.tensorflow.AttrValueR\x0cdefaultValue\x12\x20\n\x0bdescri\
    ption\x18\x04\x20\x01(\tR\x0bdescription\x12\x1f\n\x0bhas_minimum\x18\
    \x05\x20\x01(\x08R\nhasMinimum\x12\x18\n\x07minimum\x18\x06\x20\x01(\x03\
    R\x07minimum\x12<\n\x0eallowed_values\x18\x07\x20\x01(\x0b2\x15.tensorfl\
    ow.AttrValueR\rallowedValues\"K\n\rOpDeprecation\x12\x18\n\x07version\
    \x18\x01\x20\x01(\x05R\x07version\x12\x20\n\x0bexplanation\x18\x02\x20\
    \x01(\tR\x0bexplanation\"+\n\x06OpList\x12!\n\x02op\x18\x01\x20\x03(\x0b\
    2\x11.tensorflow.OpDefR\x02opB,\n\x18org.tensorflow.frameworkB\x0bOpDefP\
    rotosP\x01\xf8\x01\x01b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}