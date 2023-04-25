# Client Readiness Checklist
A place to define a "readiness checklist" for all Polywrap clients to adhere to.

## Checklist

| Feature :heavy_check_mark: | Scenario :thought_balloon: | Required :question: | Spec :clipboard: | JS :scroll: |  
|-|-|-|-|-|  
| **`wrap://` URIs** | | | | |  
| | Create & sanitize a `wrap://` URI | Yes | [->](./specs/uri.yaml) | [:heavy_check_mark:](./clients/js/src/features/uri.ts) |  
| **Client Configuration** | | | | |  
| | Add a wrap package | Yes | [->](./specs/config_embed_wrap_package.yaml) | [:heavy_check_mark:](./clients/js/src/features/config_embed_wrap_package.ts) |  
| | Add a plugin package | Yes | [->](./specs/config_plugin_package.yaml) | [:heavy_check_mark:](./clients/js/src/features/config_plugin_package.ts) |  
| | Add a plugin instance | Yes | [->](./specs/config_plugin_instance.yaml) | [:heavy_check_mark:](./clients/js/src/features/config_plugin_instance.ts) |  
| | Add a URI redirect | Yes | [->](./specs/config_uri_redirect.yaml) | [:heavy_check_mark:](./clients/js/src/features/config_uri_redirect.ts) |  
| | Add env variables | Yes | [->](./specs/config_env_variables.yaml) | [:heavy_check_mark:](./clients/js/src/features/config_env_variables.ts) |  
| | Add interface implementations | Yes | [->](./specs/config_interface_implementations.yaml) | [:heavy_check_mark:](./clients/js/src/features/config_interface_implementations.ts) |  
| | Add resolver | Yes | [->](./specs/config_resolver.yaml) | [:heavy_check_mark:](./clients/js/src/features/config_resolver.ts) |  
| | Add resolver ext | Yes | [->](./specs/config_resolver_ext.yaml) | [:heavy_check_mark:](./clients/js/src/features/config_resolver_ext.ts) |  
| **Invocation** | | | | |  
| | Invoke a wrap function<br/>(Wasm Runtime v0.1) | Yes | [->](./specs/invoke_wrap_wasm_v0_1.yaml) | [:heavy_check_mark:](./clients/js/src/features/invoke_wrap_wasm_v0_1.ts) |  
| | Invoke a plugin function | Yes | [->](./specs/invoke_plugin.yaml) | [:heavy_check_mark:](./clients/js/src/features/invoke_plugin.ts) |  
| | Subinvoke: `wrap -> wrap` | Yes | [->](./specs/subinvoke_wrap_wrap.yaml) TODO | :x: |  
| | Subinvoke: `wrap -> plugin` | Yes | [->](./specs/subinvoke_wrap_plugin.yaml) TODO | :x: |  
| | Subinvoke: `plugin -> wrap` | Yes | [->](./specs/subinvoke_plugin_wrap.yaml) TODO | :x: |  
| **`wrap://` Resolution** | | | | |  
| | Resolve a wrap package | Yes | [->](./specs/resolve_package.yaml) TODO | :x: |  
| | Resolve a wrap instance | Yes | [->](./specs/resolve_instance.yaml) TODO | :x: |  
| | Resolve a redirect | Yes | [->](./specs/resolve_redirect.yaml) TODO | :x: |  
| | Resolve `wrap://http/` &<br/>`wrap://https/` | Yes | [->](./specs/resolve_http.yaml) TODO | :x: |  
| | Resolve `wrap://file/` &<br/>`wrap://fs/` | Yes | [->](./specs/resolve_file.yaml) TODO | :x: |  
| | Resolve `wrap://ipfs/` | Yes | [->](./specs/resolve_ipfs.yaml) TODO | :x: |  
| | Resolve `wrap://ens/` contenthash | No | [->](./specs/resolve_ens_contenthash.yaml) TODO | :x: |  
| | Resolve `wrap://ens/` text-records | No | [->](./specs/resolve_ens_text_record.yaml) TODO | :x: |  
| **[WRAP Test Harness](https://github.com/polywrap/wrap-test-harness/tree/master/cases)** | | | | |  
| | `asyncify` test wrap | Yes | [->](./specs/wrap_test_harness_asyncify.yaml) TODO | :x: |  
| | `bigint-type` test wrap | Yes | [->](./specs/wrap_test_harness_bigint_type.yaml) TODO | :x: |  
| | `bignumber-type` test wrap | Yes | [->](./specs/wrap_test_harness_bignumber_type.yaml) TODO | :x: |  
| | `bytes-type` test wrap | Yes | [->](./specs/wrap_test_harness_bytes_type.yaml) TODO | :x: |  
| | `enum-type` test wrap | Yes | [->](./specs/wrap_test_harness_enum_type.yaml) TODO | :x: |  
| | `env-type` test wrap | Yes | [->](./specs/wrap_test_harness_env_type.yaml) TODO | :x: |  
| | `interface-invoke` test wrap | Yes | [->](./specs/wrap_test_harness_interface_invoke.yaml) TODO | :x: |  
| | `invalid-type` test wrap | Yes | [->](./specs/wrap_test_harness_invalid_type.yaml) TODO | :x: |  
| | `json-type` test wrap | Yes | [->](./specs/wrap_test_harness_json_type.yaml) TODO | :x: |  
| | `map-type` test wrap | Yes | [->](./specs/wrap_test_harness_map_type.yaml) TODO | :x: |  
| | `numbers-type` test wrap | Yes | [->](./specs/wrap_test_harness_numbers_type.yaml) TODO | :x: |  
| | `object-type` test wrap | Yes | [->](./specs/wrap_test_harness_object_type.yaml) TODO | :x: |  
| | `resolver` test wrap | Yes | [->](./specs/wrap_test_harness_resolver.yaml) TODO | :x: |  
| | `subinvoke` test wrap | Yes | [->](./specs/wrap_test_harness_subinvoke.yaml) TODO | :x: |  
