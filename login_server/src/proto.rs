use tonic::include_proto;

include_proto!("auth");
include_proto!("user");

pub (crate) const FILE_DESCRIPTOR_SET: &[u8] = 
    tonic::include_file_descriptor_set!("descriptor");
