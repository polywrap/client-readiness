# Client Readiness Checklist
A place to define a "readiness checklist" for all Polywrap clients to adhere to.

## Checklist

| Feature :heavy_check_mark: | Scenario :thought_balloon: | Required :question: | Spec :clipboard: |  
|-|-|-|-|  
| **`wrap://` URIs** | | | |  
| | Create & sanitize a `wrap://` URI | Yes | [uri](./specs/uri.yaml) |  
| **Client Configuration** | | | |  
| | Add a wrap package | Yes | [config_wrap_package](./specs/config_embed_wrap_package.yaml) |  
| | Add a plugin package | Yes | [config_plugin_package](./specs/config_plugin_package.yaml) |  
| | Add a plugin instance | Yes | [config_plugin_instance](./specs/config_plugin_instance.yaml) |  
| | Add a URI redirect | Yes | [config_uri_redirect](./specs/config_uri_redirect.yaml) |  
| | Add env variables | Yes | [config_env_variables](./specs/config_env_variables.yaml) |  
| | Add interface implementations | Yes | [config_interface_implementations](./specs/config_interface_implementations.yaml) |  
| | Add resolver | Yes | [config_resolver](./specs/config_resolver.yaml) |  
| | Add resolver ext | Yes | [config_resolver_ext](./specs/config_resolver_ext.yaml) |  
| **Invocation** | | | |  
| | Invoke a wrap function (Wasm Runtime v0.1) | Yes | [invoke_wrap_wasm_v0_1](./specs/invoke_wrap_wasm_v0_1.yaml) |  
| | Invoke a plugin function | Yes | [invoke_plugin](./specs/invoke_plugin.yaml) |  
| | Subinvoke: `wrap -> wrap` | Yes | [subinvoke_wrap_wrap](./specs/subinvoke_wrap_wrap.yaml) TODO |  
| | Subinvoke: `wrap -> plugin` | Yes | [subinvoke_wrap_plugin](./specs/subinvoke_wrap_plugin.yaml) TODO |  
| | Subinvoke: `plugin -> wrap` | Yes | [subinvoke_plugin_wrap](./specs/subinvoke_plugin_wrap.yaml) TODO |  
| **`wrap://` Resolution** | | | |  
| | Resolve a wrap package | Yes | [resolve_package](./specs/resolve_package.yaml) TODO |  
| | Resolve a wrap instance | Yes | [resolve_instance](./specs/resolve_instance.yaml) TODO |  
| | Resolve a redirect | Yes | [resolve_redirect](./specs/resolve_redirect.yaml) TODO |  
| | Resolve `wrap://http/...` & `wrap://https/...` | Yes | [resolve_http](./specs/resolve_http.yaml) TODO |  
| | Resolve `wrap://file/...` & `wrap://fs/...` | Yes | [resolve_file](./specs/resolve_file.yaml) TODO |  
| | Resolve `wrap://ipfs/...` | Yes | [resolve_ipfs](./specs/resolve_ipfs.yaml) TODO |  
| | Resolve `wrap://ens/...` contenthash | No | [resolve_ens_contenthash](./specs/resolve_ens_contenthash.yaml) TODO |  
| | Resolve `wrap://ens/...` text-records | No | [resolve_ens_text_records](./specs/resolve_ens_text_record.yaml) TODO |  
| **[WRAP Test Harness](https://github.com/polywrap/wrap-test-harness/tree/master/cases)** | | | |  
| | `asyncify` test wrap | Yes | [wrap_test_harness_asyncify](./specs/wrap_test_harness_asyncify.yaml) TODO |  
| | `bigint-type` test wrap | Yes | [wrap_test_harness_bigint_type](./specs/wrap_test_harness_bigint_type.yaml) TODO |  
| | `bignumber-type` test wrap | Yes | [wrap_test_harness_bignumber_type](./specs/wrap_test_harness_bignumber_type.yaml) TODO |  
| | `bytes-type` test wrap | Yes | [wrap_test_harness_bytes_type](./specs/wrap_test_harness_bytes_type.yaml) TODO |  
| | `enum-type` test wrap | Yes | [wrap_test_harness_enum_type](./specs/wrap_test_harness_enum_type.yaml) TODO |  
| | `env-type` test wrap | Yes | [wrap_test_harness_env_type](./specs/wrap_test_harness_env_type.yaml) TODO |  
| | `interface-invoke` test wrap | Yes | [wrap_test_harness_interface_invoke](./specs/wrap_test_harness_interface_invoke.yaml) TODO |  
| | `invalid-type` test wrap | Yes | [wrap_test_harness_invalid_type](./specs/wrap_test_harness_invalid_type.yaml) TODO |  
| | `json-type` test wrap | Yes | [wrap_test_harness_json_type](./specs/wrap_test_harness_json_type.yaml) TODO |  
| | `map-type` test wrap | Yes | [wrap_test_harness_map_type](./specs/wrap_test_harness_map_type.yaml) TODO |  
| | `numbers-type` test wrap | Yes | [wrap_test_harness_numbers_type](./specs/wrap_test_harness_numbers_type.yaml) TODO |  
| | `object-type` test wrap | Yes | [wrap_test_harness_object_type](./specs/wrap_test_harness_object_type.yaml) TODO |  
| | `resolver` test wrap | Yes | [wrap_test_harness_resolver](./specs/wrap_test_harness_resolver.yaml) TODO |  
| | `subinvoke` test wrap | Yes | [wrap_test_harness_subinvoke](./specs/wrap_test_harness_subinvoke.yaml) TODO |  
