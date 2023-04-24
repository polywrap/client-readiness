# Client Readiness Checklist
A place to define a "readiness checklist" for all Polywrap clients to adhere to.

## Checklist

| Feature :heavy_check_mark: | Scenario :thought_balloon: | Required :question: | Spec :clipboard: |  
|:-|:-|-:|:-|  
| **`wrap://` URIs** | | | |  
| | Create & sanitize a `wrap://` URI | Yes | [uri](./specs/uri.yaml) |  
| **Client Configuration** | | | |  
| | Add a wrap package | Yes | [config_wrap_package](./specs/config_embed_wrap_package.yaml) |  
| | Add a plugin package | Yes | [config_plugin_package](./specs/config_plugin_package.yaml) |  
| | Add a wrap instance | Yes | [config_wrap_instance](./specs/config_embed_wrap_instance.yaml) |  
| | Add a plugin instance | Yes | [config_plugin_instance](./specs/config_plugin_instance.yaml) |  
| | Add a URI redirect | Yes | [config_uri_redirect](./specs/config_uri_redirect.yaml) |  
| | Add env variables | Yes | [config_env_variables](./specs/config_env_variables.yaml) |  
| | Add interface implementations | Yes | [config_interface_implementations](./specs/config_interface_implementations.yaml) |  
| | Add resolver | Yes | [config_resolver](./specs/config_resolver.yaml) |  
| | Add resolver ext | Yes | [config_resolver_ext](./specs/config_resolver_ext.yaml) |  
| **Invocation** | | | |  
| | Invoke a wrap function (Wasm Runtime v0.1) | Yes | [invoke_wrap_wasm_v0_1](./specs/invoke_wrap_wasm_v0_1.yaml) |  
| | Invoke a plugin function | Yes | [invoke_plugin](./specs/invoke_plugin.yaml) |  
| | Subinvoke: `wrap -> wrap` | Yes | [subinvoke_wrap_wrap](./specs/subinvoke_wrap_wrap.yaml) |  
| | Subinvoke: `wrap -> plugin` | Yes | [subinvoke_wrap_plugin](./specs/subinvoke_wrap_plugin.yaml) |  
| | Subinvoke: `plugin -> wrap` | Yes | [subinvoke_plugin_wrap](./specs/subinvoke_plugin_wrap.yaml) |  
| **`wrap://` Resolution** | | | |  
| | Resolve a wrap package | Yes | [resolve_package](./specs/resolve_package.yaml) |  
| | Resolve a wrap instance | Yes | [resolve_instance](./specs/resolve_instance.yaml) |  
| | Resolve a redirect | Yes | [resolve_redirect](./specs/resolve_redirect.yaml) |  
| | Resolve `wrap://http/...` & `wrap://https/...` | Yes | [resolve_http](./specs/resolve_http.yaml) |  
| | Resolve `wrap://file/...` & `wrap://fs/...` | Yes | [resolve_file](./specs/resolve_file.yaml) |  
| | Resolve `wrap://ipfs/...` | Yes | [resolve_ipfs](./specs/resolve_ipfs.yaml) |  
| | Resolve `wrap://ens/...` content records | No | [resolve_ens_content_records](./specs/resolve_ens_content_record.yaml) |  
| | Resolve `wrap://ens/...` text records | No | [resolve_ens_text_records](./specs/resolve_ens_text_record.yaml) |  
| **[WRAP Test Harness](https://github.com/polywrap/wrap-test-harness/tree/master/cases)** | | | |  
| | `asyncify` test wrap | Yes | [wrap_test_harness_asyncify](./specs/wrap_test_harness_asyncify.yaml) |  
| | `bigint-type` test wrap | Yes | [wrap_test_harness_bigint_type](./specs/wrap_test_harness_bigint_type.yaml) |  
| | `bignumber-type` test wrap | Yes | [wrap_test_harness_bignumber_type](./specs/wrap_test_harness_bignumber_type.yaml) |  
| | `bytes-type` test wrap | Yes | [wrap_test_harness_bytes_type](./specs/wrap_test_harness_bytes_type.yaml) |  
| | `enum-type` test wrap | Yes | [wrap_test_harness_enum_type](./specs/wrap_test_harness_enum_type.yaml) |  
| | `env-type` test wrap | Yes | [wrap_test_harness_env_type](./specs/wrap_test_harness_env_type.yaml) |  
| | `interface-invoke` test wrap | Yes | [wrap_test_harness_interface_invoke](./specs/wrap_test_harness_interface_invoke.yaml) |  
| | `invalid-type` test wrap | Yes | [wrap_test_harness_invalid_type](./specs/wrap_test_harness_invalid_type.yaml) |  
| | `json-type` test wrap | Yes | [wrap_test_harness_json_type](./specs/wrap_test_harness_json_type.yaml) |  
| | `map-type` test wrap | Yes | [wrap_test_harness_map_type](./specs/wrap_test_harness_map_type.yaml) |  
| | `numbers-type` test wrap | Yes | [wrap_test_harness_numbers_type](./specs/wrap_test_harness_numbers_type.yaml) |  
| | `object-type` test wrap | Yes | [wrap_test_harness_object_type](./specs/wrap_test_harness_object_type.yaml) |  
| | `resolver` test wrap | Yes | [wrap_test_harness_resolver](./specs/wrap_test_harness_resolver.yaml) |  
| | `subinvoke` test wrap | Yes | [wrap_test_harness_subinvoke](./specs/wrap_test_harness_subinvoke.yaml) |  
| **Data Translation** | | | |  
| | Encode `(U)Int(8\|16\|32\|)` to MsgPack | Yes | [dt_encode_ints_msgpack](./specs/dt_encode_ints_msgpack.yaml) |  
| | Decode `pos/neg fixint` & `(u)int 8\|16\|32` from MsgPack | Yes | [dt_decode_ints_msgpack](./specs/dt_decode_ints_msgpack.yaml) |  
| | Encode `String` to MsgPack | Yes | [dt_encode_string_msgpack](./specs/dt_encode_string_msgpack.yaml) |  
| | Decode `fixstr` & `str 8\|16\|32` from MsgPack | Yes | [dt_decode_string_msgpack](./specs/dt_decode_string_msgpack.yaml) |  
| | Encode `Boolean` to MsgPack | Yes | [dt_encode_boolean_msgpack](./specs/dt_encode_boolean_msgpack.yaml) |  
| | Decode `true` & `false` from MsgPack | Yes | [dt_decode_boolean_msgpack](./specs/dt_decode_boolean_msgpack.yaml) |  
| | Encode `Bytes` to MsgPack | Yes | [dt_encode_bytes_msgpack](./specs/dt_encode_bytes_msgpack.yaml) |  
| | Decode `bin 8\|16\|32` from MsgPack | Yes | [dt_decode_bytes_msgpack](./specs/dt_decode_bytes_msgpack.yaml) |  
| | Encode `Array<T>` to MsgPack | Yes | [dt_encode_array_msgpack](./specs/dt_encode_array_msgpack.yaml) |  
| | Decode `fixarray` & `array 16\|32` from MsgPack | Yes | [dt_decode_array_msgpack](./specs/dt_decode_array_msgpack.yaml) |  
| | Encode `Object` to MsgPack | Yes | [dt_encode_object_msgpack](./specs/dt_encode_object_msgpack.yaml) |  
| | Decode `Object` (`fixmap` & `map 16\|32`) from MsgPack | Yes | [dt_decode_object_msgpack](./specs/dt_decode_object_msgpack.yaml) |  
| | Encode `Map<K, V>` to MsgPack | Yes | [dt_encode_map_msgpack](./specs/dt_encode_map_msgpack.yaml) |  
| | Decode `Map<K, V>` (`ext 8\|16\|32` type `0x01`) from MsgPack | Yes | [dt_decode_map_msgpack](./spec/dt_decode_map_msgpack.yaml) |  
