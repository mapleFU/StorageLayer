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
    pub zmq_host: ::std::string::String,
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

    // string zmq_host = 2;


    pub fn get_zmq_host(&self) -> &str {
        &self.zmq_host
    }
    pub fn clear_zmq_host(&mut self) {
        self.zmq_host.clear();
    }

    // Param is passed by value, moved
    pub fn set_zmq_host(&mut self, v: ::std::string::String) {
        self.zmq_host = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_zmq_host(&mut self) -> &mut ::std::string::String {
        &mut self.zmq_host
    }

    // Take field
    pub fn take_zmq_host(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.zmq_host, ::std::string::String::new())
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.zmq_host)?;
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
        if !self.zmq_host.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.zmq_host);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.zmq_tcp_port.is_empty() {
            os.write_string(1, &self.zmq_tcp_port)?;
        }
        if !self.zmq_host.is_empty() {
            os.write_string(2, &self.zmq_host)?;
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
                    "zmq_host",
                    |m: &Server| { &m.zmq_host },
                    |m: &mut Server| { &mut m.zmq_host },
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
        self.zmq_host.clear();
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
    \n\x08zk.proto\"E\n\x06Server\x12\x20\n\x0czmq_tcp_port\x18\x01\x20\x01(\
    \tR\nzmqTcpPort\x12\x19\n\x08zmq_host\x18\x02\x20\x01(\tR\x07zmqHostJ\
    \xd6\x01\n\x06\x12\x04\0\0\x07\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\
    \x02\x04\0\x12\x04\x02\0\x07\x01\n\n\n\x03\x04\0\x01\x12\x03\x02\x08\x0e\
    \n\x1b\n\x04\x04\0\x02\0\x12\x03\x04\x04\x1c\x1a\x0e\x20addr\x20for\x20z\
    mq\n\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x04\x04\x02\x10\n\x0c\n\x05\x04\0\
    \x02\0\x05\x12\x03\x04\x04\n\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x04\x0b\
    \x17\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x04\x1a\x1b\n\x1b\n\x04\x04\0\
    \x02\x01\x12\x03\x06\x04\x18\x1a\x0e\x20host\x20for\x20zmq\n\n\r\n\x05\
    \x04\0\x02\x01\x04\x12\x04\x06\x04\x04\x1c\n\x0c\n\x05\x04\0\x02\x01\x05\
    \x12\x03\x06\x04\n\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x06\x0b\x13\n\
    \x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x06\x16\x17b\x06proto3\
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
