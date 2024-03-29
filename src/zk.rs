// This file is generated by rust-protobuf 2.5.0. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

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

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Server {
    // message fields
    pub zmq_tcp_port: ::std::string::String,
    pub host: ::std::string::String,
    pub total_space: u64,
    pub available_space: u64,
    pub http_port: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Server {
    fn default() -> &'a Server {
        <Server as ::protobuf::Message>::default_instance()
    }
}

impl Server {
    pub fn new() -> Server {
        ::std::default::Default::default()
    }

    // string zmq_tcp_port = 1;


    pub fn get_zmq_tcp_port(&self) -> &str {
        &self.zmq_tcp_port
    }
    pub fn clear_zmq_tcp_port(&mut self) {
        self.zmq_tcp_port.clear();
    }

    // Param is passed by value, moved
    pub fn set_zmq_tcp_port(&mut self, v: ::std::string::String) {
        self.zmq_tcp_port = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_zmq_tcp_port(&mut self) -> &mut ::std::string::String {
        &mut self.zmq_tcp_port
    }

    // Take field
    pub fn take_zmq_tcp_port(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.zmq_tcp_port, ::std::string::String::new())
    }

    // string host = 2;


    pub fn get_host(&self) -> &str {
        &self.host
    }
    pub fn clear_host(&mut self) {
        self.host.clear();
    }

    // Param is passed by value, moved
    pub fn set_host(&mut self, v: ::std::string::String) {
        self.host = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_host(&mut self) -> &mut ::std::string::String {
        &mut self.host
    }

    // Take field
    pub fn take_host(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.host, ::std::string::String::new())
    }

    // uint64 total_space = 3;


    pub fn get_total_space(&self) -> u64 {
        self.total_space
    }
    pub fn clear_total_space(&mut self) {
        self.total_space = 0;
    }

    // Param is passed by value, moved
    pub fn set_total_space(&mut self, v: u64) {
        self.total_space = v;
    }

    // uint64 available_space = 4;


    pub fn get_available_space(&self) -> u64 {
        self.available_space
    }
    pub fn clear_available_space(&mut self) {
        self.available_space = 0;
    }

    // Param is passed by value, moved
    pub fn set_available_space(&mut self, v: u64) {
        self.available_space = v;
    }

    // string http_port = 5;


    pub fn get_http_port(&self) -> &str {
        &self.http_port
    }
    pub fn clear_http_port(&mut self) {
        self.http_port.clear();
    }

    // Param is passed by value, moved
    pub fn set_http_port(&mut self, v: ::std::string::String) {
        self.http_port = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_http_port(&mut self) -> &mut ::std::string::String {
        &mut self.http_port
    }

    // Take field
    pub fn take_http_port(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.http_port, ::std::string::String::new())
    }
}

impl ::protobuf::Message for Server {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.zmq_tcp_port)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.host)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.total_space = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.available_space = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.http_port)?;
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
        if !self.zmq_tcp_port.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.zmq_tcp_port);
        }
        if !self.host.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.host);
        }
        if self.total_space != 0 {
            my_size += ::protobuf::rt::value_size(3, self.total_space, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.available_space != 0 {
            my_size += ::protobuf::rt::value_size(4, self.available_space, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.http_port.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.http_port);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.zmq_tcp_port.is_empty() {
            os.write_string(1, &self.zmq_tcp_port)?;
        }
        if !self.host.is_empty() {
            os.write_string(2, &self.host)?;
        }
        if self.total_space != 0 {
            os.write_uint64(3, self.total_space)?;
        }
        if self.available_space != 0 {
            os.write_uint64(4, self.available_space)?;
        }
        if !self.http_port.is_empty() {
            os.write_string(5, &self.http_port)?;
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
        Self::descriptor_static()
    }

    fn new() -> Server {
        Server::new()
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
                    "zmq_tcp_port",
                    |m: &Server| { &m.zmq_tcp_port },
                    |m: &mut Server| { &mut m.zmq_tcp_port },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "host",
                    |m: &Server| { &m.host },
                    |m: &mut Server| { &mut m.host },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "total_space",
                    |m: &Server| { &m.total_space },
                    |m: &mut Server| { &mut m.total_space },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "available_space",
                    |m: &Server| { &m.available_space },
                    |m: &mut Server| { &mut m.available_space },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "http_port",
                    |m: &Server| { &m.http_port },
                    |m: &mut Server| { &mut m.http_port },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Server>(
                    "Server",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Server {
        static mut instance: ::protobuf::lazy::Lazy<Server> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Server,
        };
        unsafe {
            instance.get(Server::new)
        }
    }
}

impl ::protobuf::Clear for Server {
    fn clear(&mut self) {
        self.zmq_tcp_port.clear();
        self.host.clear();
        self.total_space = 0;
        self.available_space = 0;
        self.http_port.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Server {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Server {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x08zk.proto\"\xa5\x01\n\x06Server\x12\x20\n\x0czmq_tcp_port\x18\x01\
    \x20\x01(\tR\nzmqTcpPort\x12\x12\n\x04host\x18\x02\x20\x01(\tR\x04host\
    \x12\x1f\n\x0btotal_space\x18\x03\x20\x01(\x04R\ntotalSpace\x12'\n\x0fav\
    ailable_space\x18\x04\x20\x01(\x04R\x0eavailableSpace\x12\x1b\n\thttp_po\
    rt\x18\x05\x20\x01(\tR\x08httpPortJ\xe3\x03\n\x06\x12\x04\0\0\x0f\x01\n\
    \x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x04\0\x12\x04\x02\0\x0f\x01\n\n\
    \n\x03\x04\0\x01\x12\x03\x02\x08\x0e\n\x1b\n\x04\x04\0\x02\0\x12\x03\x04\
    \x04\x1c\x1a\x0e\x20addr\x20for\x20zmq\n\n\r\n\x05\x04\0\x02\0\x04\x12\
    \x04\x04\x04\x02\x10\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x04\x04\n\n\x0c\
    \n\x05\x04\0\x02\0\x01\x12\x03\x04\x0b\x17\n\x0c\n\x05\x04\0\x02\0\x03\
    \x12\x03\x04\x1a\x1b\n\x1b\n\x04\x04\0\x02\x01\x12\x03\x06\x04\x14\x1a\
    \x0e\x20host\x20for\x20zmq\n\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x06\x04\
    \x04\x1c\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x06\x04\n\n\x0c\n\x05\x04\
    \0\x02\x01\x01\x12\x03\x06\x0b\x0f\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\
    \x06\x12\x13\n\"\n\x04\x04\0\x02\x02\x12\x03\t\x04\x1b\x1a\x15\x20data\
    \x20for\x20disk\x20space\n\n\r\n\x05\x04\0\x02\x02\x04\x12\x04\t\x04\x06\
    \x14\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\t\x04\n\n\x0c\n\x05\x04\0\x02\
    \x02\x01\x12\x03\t\x0b\x16\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\t\x19\
    \x1a\n\x0b\n\x04\x04\0\x02\x03\x12\x03\n\x04\x1f\n\r\n\x05\x04\0\x02\x03\
    \x04\x12\x04\n\x04\t\x1b\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\n\x04\n\n\
    \x0c\n\x05\x04\0\x02\x03\x01\x12\x03\n\x0b\x1a\n\x0c\n\x05\x04\0\x02\x03\
    \x03\x12\x03\n\x1d\x1e\n/\n\x04\x04\0\x02\x04\x12\x03\r\x04\x19\x1a\"\
    \x20host\x20to\x20interact\x20with\x20http\x20file\x20\n\n\r\n\x05\x04\0\
    \x02\x04\x04\x12\x04\r\x04\n\x1f\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03\r\
    \x04\n\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03\r\x0b\x14\n\x0c\n\x05\x04\0\
    \x02\x04\x03\x12\x03\r\x17\x18b\x06proto3\
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
