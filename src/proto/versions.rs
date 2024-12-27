// This file is generated by rust-protobuf 3.7.1. Do not edit
// .proto file is parsed by protoc 3.21.12
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `versions.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:Versions)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Versions {
    // message fields
    // @@protoc_insertion_point(field:Versions.packages)
    pub packages: ::std::vec::Vec<VersionsPackage>,
    // @@protoc_insertion_point(field:Versions.repository)
    pub repository: ::std::option::Option<::std::string::String>,
    // special fields
    // @@protoc_insertion_point(special_field:Versions.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Versions {
    fn default() -> &'a Versions {
        <Versions as ::protobuf::Message>::default_instance()
    }
}

impl Versions {
    pub fn new() -> Versions {
        ::std::default::Default::default()
    }

    // required string repository = 2;

    pub fn repository(&self) -> &str {
        match self.repository.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_repository(&mut self) {
        self.repository = ::std::option::Option::None;
    }

    pub fn has_repository(&self) -> bool {
        self.repository.is_some()
    }

    // Param is passed by value, moved
    pub fn set_repository(&mut self, v: ::std::string::String) {
        self.repository = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_repository(&mut self) -> &mut ::std::string::String {
        if self.repository.is_none() {
            self.repository = ::std::option::Option::Some(::std::string::String::new());
        }
        self.repository.as_mut().unwrap()
    }

    // Take field
    pub fn take_repository(&mut self) -> ::std::string::String {
        self.repository.take().unwrap_or_else(|| ::std::string::String::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "packages",
            |m: &Versions| { &m.packages },
            |m: &mut Versions| { &mut m.packages },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "repository",
            |m: &Versions| { &m.repository },
            |m: &mut Versions| { &mut m.repository },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Versions>(
            "Versions",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Versions {
    const NAME: &'static str = "Versions";

    fn is_initialized(&self) -> bool {
        if self.repository.is_none() {
            return false;
        }
        for v in &self.packages {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.packages.push(is.read_message()?);
                },
                18 => {
                    self.repository = ::std::option::Option::Some(is.read_string()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        for value in &self.packages {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.repository.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.packages {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        if let Some(v) = self.repository.as_ref() {
            os.write_string(2, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Versions {
        Versions::new()
    }

    fn clear(&mut self) {
        self.packages.clear();
        self.repository = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Versions {
        static instance: Versions = Versions {
            packages: ::std::vec::Vec::new(),
            repository: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Versions {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Versions").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Versions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Versions {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:VersionsPackage)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct VersionsPackage {
    // message fields
    // @@protoc_insertion_point(field:VersionsPackage.name)
    pub name: ::std::option::Option<::std::string::String>,
    // @@protoc_insertion_point(field:VersionsPackage.versions)
    pub versions: ::std::vec::Vec<::std::string::String>,
    // @@protoc_insertion_point(field:VersionsPackage.retired)
    pub retired: ::std::vec::Vec<i32>,
    // special fields
    // @@protoc_insertion_point(special_field:VersionsPackage.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a VersionsPackage {
    fn default() -> &'a VersionsPackage {
        <VersionsPackage as ::protobuf::Message>::default_instance()
    }
}

impl VersionsPackage {
    pub fn new() -> VersionsPackage {
        ::std::default::Default::default()
    }

    // required string name = 1;

    pub fn name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_name(&mut self) {
        self.name = ::std::option::Option::None;
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name = ::std::option::Option::Some(::std::string::String::new());
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "name",
            |m: &VersionsPackage| { &m.name },
            |m: &mut VersionsPackage| { &mut m.name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "versions",
            |m: &VersionsPackage| { &m.versions },
            |m: &mut VersionsPackage| { &mut m.versions },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "retired",
            |m: &VersionsPackage| { &m.retired },
            |m: &mut VersionsPackage| { &mut m.retired },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<VersionsPackage>(
            "VersionsPackage",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for VersionsPackage {
    const NAME: &'static str = "VersionsPackage";

    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.name = ::std::option::Option::Some(is.read_string()?);
                },
                18 => {
                    self.versions.push(is.read_string()?);
                },
                26 => {
                    is.read_repeated_packed_int32_into(&mut self.retired)?;
                },
                24 => {
                    self.retired.push(is.read_int32()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        for value in &self.versions {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::vec_packed_int32_size(3, &self.retired);
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.name.as_ref() {
            os.write_string(1, v)?;
        }
        for v in &self.versions {
            os.write_string(2, &v)?;
        };
        os.write_repeated_packed_int32(3, &self.retired)?;
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> VersionsPackage {
        VersionsPackage::new()
    }

    fn clear(&mut self) {
        self.name = ::std::option::Option::None;
        self.versions.clear();
        self.retired.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static VersionsPackage {
        static instance: VersionsPackage = VersionsPackage {
            name: ::std::option::Option::None,
            versions: ::std::vec::Vec::new(),
            retired: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for VersionsPackage {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("VersionsPackage").unwrap()).clone()
    }
}

impl ::std::fmt::Display for VersionsPackage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VersionsPackage {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0eversions.proto\"X\n\x08Versions\x12,\n\x08packages\x18\x01\x20\x03\
    (\x0b2\x10.VersionsPackageR\x08packages\x12\x1e\n\nrepository\x18\x02\
    \x20\x02(\tR\nrepository\"_\n\x0fVersionsPackage\x12\x12\n\x04name\x18\
    \x01\x20\x02(\tR\x04name\x12\x1a\n\x08versions\x18\x02\x20\x03(\tR\x08ve\
    rsions\x12\x1c\n\x07retired\x18\x03\x20\x03(\x05R\x07retiredB\x02\x10\
    \x01\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(Versions::generated_message_descriptor_data());
            messages.push(VersionsPackage::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
