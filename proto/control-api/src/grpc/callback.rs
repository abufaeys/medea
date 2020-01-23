// This file is generated by rust-protobuf 2.10.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `callback.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_10_1;

#[derive(PartialEq,Clone,Default)]
pub struct Request {
    // message fields
    pub fid: ::std::string::String,
    pub at: ::std::string::String,
    // message oneof groups
    pub event: ::std::option::Option<Request_oneof_event>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Request {
    fn default() -> &'a Request {
        <Request as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum Request_oneof_event {
    on_join(OnJoin),
    on_leave(OnLeave),
}

impl Request {
    pub fn new() -> Request {
        ::std::default::Default::default()
    }

    // string fid = 1;


    pub fn get_fid(&self) -> &str {
        &self.fid
    }
    pub fn clear_fid(&mut self) {
        self.fid.clear();
    }

    // Param is passed by value, moved
    pub fn set_fid(&mut self, v: ::std::string::String) {
        self.fid = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fid(&mut self) -> &mut ::std::string::String {
        &mut self.fid
    }

    // Take field
    pub fn take_fid(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.fid, ::std::string::String::new())
    }

    // string at = 2;


    pub fn get_at(&self) -> &str {
        &self.at
    }
    pub fn clear_at(&mut self) {
        self.at.clear();
    }

    // Param is passed by value, moved
    pub fn set_at(&mut self, v: ::std::string::String) {
        self.at = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_at(&mut self) -> &mut ::std::string::String {
        &mut self.at
    }

    // Take field
    pub fn take_at(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.at, ::std::string::String::new())
    }

    // .medea_callback.OnJoin on_join = 3;


    pub fn get_on_join(&self) -> &OnJoin {
        match self.event {
            ::std::option::Option::Some(Request_oneof_event::on_join(ref v)) => v,
            _ => OnJoin::default_instance(),
        }
    }
    pub fn clear_on_join(&mut self) {
        self.event = ::std::option::Option::None;
    }

    pub fn has_on_join(&self) -> bool {
        match self.event {
            ::std::option::Option::Some(Request_oneof_event::on_join(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_on_join(&mut self, v: OnJoin) {
        self.event = ::std::option::Option::Some(Request_oneof_event::on_join(v))
    }

    // Mutable pointer to the field.
    pub fn mut_on_join(&mut self) -> &mut OnJoin {
        if let ::std::option::Option::Some(Request_oneof_event::on_join(_)) = self.event {
        } else {
            self.event = ::std::option::Option::Some(Request_oneof_event::on_join(OnJoin::new()));
        }
        match self.event {
            ::std::option::Option::Some(Request_oneof_event::on_join(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_on_join(&mut self) -> OnJoin {
        if self.has_on_join() {
            match self.event.take() {
                ::std::option::Option::Some(Request_oneof_event::on_join(v)) => v,
                _ => panic!(),
            }
        } else {
            OnJoin::new()
        }
    }

    // .medea_callback.OnLeave on_leave = 4;


    pub fn get_on_leave(&self) -> &OnLeave {
        match self.event {
            ::std::option::Option::Some(Request_oneof_event::on_leave(ref v)) => v,
            _ => OnLeave::default_instance(),
        }
    }
    pub fn clear_on_leave(&mut self) {
        self.event = ::std::option::Option::None;
    }

    pub fn has_on_leave(&self) -> bool {
        match self.event {
            ::std::option::Option::Some(Request_oneof_event::on_leave(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_on_leave(&mut self, v: OnLeave) {
        self.event = ::std::option::Option::Some(Request_oneof_event::on_leave(v))
    }

    // Mutable pointer to the field.
    pub fn mut_on_leave(&mut self) -> &mut OnLeave {
        if let ::std::option::Option::Some(Request_oneof_event::on_leave(_)) = self.event {
        } else {
            self.event = ::std::option::Option::Some(Request_oneof_event::on_leave(OnLeave::new()));
        }
        match self.event {
            ::std::option::Option::Some(Request_oneof_event::on_leave(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_on_leave(&mut self) -> OnLeave {
        if self.has_on_leave() {
            match self.event.take() {
                ::std::option::Option::Some(Request_oneof_event::on_leave(v)) => v,
                _ => panic!(),
            }
        } else {
            OnLeave::new()
        }
    }
}

impl ::protobuf::Message for Request {
    fn is_initialized(&self) -> bool {
        if let Some(Request_oneof_event::on_join(ref v)) = self.event {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_event::on_leave(ref v)) = self.event {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.fid)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.at)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.event = ::std::option::Option::Some(Request_oneof_event::on_join(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.event = ::std::option::Option::Some(Request_oneof_event::on_leave(is.read_message()?));
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
        if !self.fid.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.fid);
        }
        if !self.at.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.at);
        }
        if let ::std::option::Option::Some(ref v) = self.event {
            match v {
                &Request_oneof_event::on_join(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_event::on_leave(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.fid.is_empty() {
            os.write_string(1, &self.fid)?;
        }
        if !self.at.is_empty() {
            os.write_string(2, &self.at)?;
        }
        if let ::std::option::Option::Some(ref v) = self.event {
            match v {
                &Request_oneof_event::on_join(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_event::on_leave(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Request {
        Request::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "fid",
                    |m: &Request| { &m.fid },
                    |m: &mut Request| { &mut m.fid },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "at",
                    |m: &Request| { &m.at },
                    |m: &mut Request| { &mut m.at },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, OnJoin>(
                    "on_join",
                    Request::has_on_join,
                    Request::get_on_join,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, OnLeave>(
                    "on_leave",
                    Request::has_on_leave,
                    Request::get_on_leave,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Request>(
                    "Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Request {
        static mut instance: ::protobuf::lazy::Lazy<Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Request,
        };
        unsafe {
            instance.get(Request::new)
        }
    }
}

impl ::protobuf::Clear for Request {
    fn clear(&mut self) {
        self.fid.clear();
        self.at.clear();
        self.event = ::std::option::Option::None;
        self.event = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Response {
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Response {
    fn default() -> &'a Response {
        <Response as ::protobuf::Message>::default_instance()
    }
}

impl Response {
    pub fn new() -> Response {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for Response {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Response {
        Response::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<Response>(
                    "Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Response {
        static mut instance: ::protobuf::lazy::Lazy<Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Response,
        };
        unsafe {
            instance.get(Response::new)
        }
    }
}

impl ::protobuf::Clear for Response {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OnJoin {
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a OnJoin {
    fn default() -> &'a OnJoin {
        <OnJoin as ::protobuf::Message>::default_instance()
    }
}

impl OnJoin {
    pub fn new() -> OnJoin {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for OnJoin {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> OnJoin {
        OnJoin::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<OnJoin>(
                    "OnJoin",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static OnJoin {
        static mut instance: ::protobuf::lazy::Lazy<OnJoin> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OnJoin,
        };
        unsafe {
            instance.get(OnJoin::new)
        }
    }
}

impl ::protobuf::Clear for OnJoin {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OnJoin {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OnJoin {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OnLeave {
    // message fields
    pub reason: OnLeave_Reason,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a OnLeave {
    fn default() -> &'a OnLeave {
        <OnLeave as ::protobuf::Message>::default_instance()
    }
}

impl OnLeave {
    pub fn new() -> OnLeave {
        ::std::default::Default::default()
    }

    // .medea_callback.OnLeave.Reason reason = 1;


    pub fn get_reason(&self) -> OnLeave_Reason {
        self.reason
    }
    pub fn clear_reason(&mut self) {
        self.reason = OnLeave_Reason::DISCONNECTED;
    }

    // Param is passed by value, moved
    pub fn set_reason(&mut self, v: OnLeave_Reason) {
        self.reason = v;
    }
}

impl ::protobuf::Message for OnLeave {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.reason, 1, &mut self.unknown_fields)?
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
        if self.reason != OnLeave_Reason::DISCONNECTED {
            my_size += ::protobuf::rt::enum_size(1, self.reason);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.reason != OnLeave_Reason::DISCONNECTED {
            os.write_enum(1, self.reason.value())?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> OnLeave {
        OnLeave::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<OnLeave_Reason>>(
                    "reason",
                    |m: &OnLeave| { &m.reason },
                    |m: &mut OnLeave| { &mut m.reason },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OnLeave>(
                    "OnLeave",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static OnLeave {
        static mut instance: ::protobuf::lazy::Lazy<OnLeave> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OnLeave,
        };
        unsafe {
            instance.get(OnLeave::new)
        }
    }
}

impl ::protobuf::Clear for OnLeave {
    fn clear(&mut self) {
        self.reason = OnLeave_Reason::DISCONNECTED;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OnLeave {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OnLeave {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum OnLeave_Reason {
    DISCONNECTED = 0,
    LOST_CONNECTION = 1,
    SERVER_SHUTDOWN = 2,
}

impl ::protobuf::ProtobufEnum for OnLeave_Reason {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<OnLeave_Reason> {
        match value {
            0 => ::std::option::Option::Some(OnLeave_Reason::DISCONNECTED),
            1 => ::std::option::Option::Some(OnLeave_Reason::LOST_CONNECTION),
            2 => ::std::option::Option::Some(OnLeave_Reason::SERVER_SHUTDOWN),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [OnLeave_Reason] = &[
            OnLeave_Reason::DISCONNECTED,
            OnLeave_Reason::LOST_CONNECTION,
            OnLeave_Reason::SERVER_SHUTDOWN,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("OnLeave_Reason", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for OnLeave_Reason {
}

impl ::std::default::Default for OnLeave_Reason {
    fn default() -> Self {
        OnLeave_Reason::DISCONNECTED
    }
}

impl ::protobuf::reflect::ProtobufValue for OnLeave_Reason {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0ecallback.proto\x12\x0emedea_callback\"\x9d\x01\n\x07Request\x12\
    \x10\n\x03fid\x18\x01\x20\x01(\tR\x03fid\x12\x0e\n\x02at\x18\x02\x20\x01\
    (\tR\x02at\x121\n\x07on_join\x18\x03\x20\x01(\x0b2\x16.medea_callback.On\
    JoinH\0R\x06onJoin\x124\n\x08on_leave\x18\x04\x20\x01(\x0b2\x17.medea_ca\
    llback.OnLeaveH\0R\x07onLeaveB\x07\n\x05event\"\n\n\x08Response\"\x08\n\
    \x06OnJoin\"\x87\x01\n\x07OnLeave\x126\n\x06reason\x18\x01\x20\x01(\x0e2\
    \x1e.medea_callback.OnLeave.ReasonR\x06reason\"D\n\x06Reason\x12\x10\n\
    \x0cDISCONNECTED\x10\0\x12\x13\n\x0fLOST_CONNECTION\x10\x01\x12\x13\n\
    \x0fSERVER_SHUTDOWN\x10\x022H\n\x08Callback\x12<\n\x07OnEvent\x12\x17.me\
    dea_callback.Request\x1a\x18.medea_callback.Responseb\x06proto3\
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