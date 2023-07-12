use polywrap_client::wrap_manifest::versions::{WrapManifest01, WrapManifest01Abi};

pub fn get_default_manifest() -> WrapManifest01 {
  WrapManifest01 {
    abi: WrapManifest01Abi {
        enum_types: None,
        env_type: None,
        imported_enum_types: None,
        imported_env_types: None,
        imported_module_types: None,
        imported_object_types: None,
        interface_types: None,
        module_type: None,
        object_types: None,
        version: None,
    },
    name: String::from(""),
    type_: String::from(""),
    version: String::from(""),
  }
}