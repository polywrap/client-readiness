# Client Readiness Checklist
A place to define a "readiness checklist" for all Polywrap clients to adhere to.

## Checklist

| Feature :heavy_check_mark: | Scenario :thought_balloon: | Required :question: | Spec :clipboard: | JS :scroll: |  
|-|-|-|-|-|  
| **`wrap://` URIs** | | | | |  
| | Create & sanitize a `wrap://` URI | Yes | [:mag:](./specs/uri.yaml) | [:heavy_check_mark:](./clients/js/src/features/uri.ts) |  
| **Client Configuration** | | | | |  
| | Add a wrap package | Yes | [:mag:](./specs/config_embed_wrap_package.yaml) | [:heavy_check_mark:](./clients/js/src/features/config_embed_wrap_package.ts) |  
| | Add a plugin package | Yes | [:mag:](./specs/config_plugin_package.yaml) | [:heavy_check_mark:](./clients/js/src/features/config_plugin_package.ts) |  
| | Add a plugin instance | Yes | [:mag:](./specs/config_plugin_instance.yaml) | [:heavy_check_mark:](./clients/js/src/features/config_plugin_instance.ts) |  
| | Add a URI redirect | Yes | [:mag:](./specs/config_uri_redirect.yaml) | [:heavy_check_mark:](./clients/js/src/features/config_uri_redirect.ts) |  
| | Add env variables | Yes | [:mag:](./specs/config_env_variables.yaml) | [:heavy_check_mark:](./clients/js/src/features/config_env_variables.ts) |  
| | Add interface implementations | Yes | [:mag:](./specs/config_interface_implementations.yaml) | [:heavy_check_mark:](./clients/js/src/features/config_interface_implementations.ts) |  
| | Add resolver | Yes | [:mag:](./specs/config_resolver.yaml) | [:heavy_check_mark:](./clients/js/src/features/config_resolver.ts) |  
| | Add resolver ext | Yes | [:mag:](./specs/config_resolver_ext.yaml) | [:heavy_check_mark:](./clients/js/src/features/config_resolver_ext.ts) |  
| **Invocation** | | | | |  
| | Invoke a wrap function<br/>(Wasm Runtime v0.1) | Yes | [:mag:](./specs/invoke_wrap_wasm_v0_1.yaml) | [:heavy_check_mark:](./clients/js/src/features/invoke_wrap_wasm_v0_1.ts) |  
| | Invoke a plugin function | Yes | [:mag:](./specs/invoke_plugin.yaml) | [:heavy_check_mark:](./clients/js/src/features/invoke_plugin.ts) |  
| | Subinvoke: `wrap -> wrap` | Yes | [:mag:](./specs/subinvoke_wrap_wrap.yaml) | [:heavy_check_mark:](./clients/js/src/features/subinvoke_wrap_wrap.ts) |  
| | Subinvoke: `wrap -> plugin` | Yes | [:mag:](./specs/subinvoke_wrap_plugin.yaml) | [:heavy_check_mark:](./clients/js/src/features/subinvoke_wrap_plugin.ts) |  
| | Subinvoke: `plugin -> wrap` | Yes | [:mag:](./specs/subinvoke_plugin_wrap.yaml) | [:heavy_check_mark:](./clients/js/src/features/subinvoke_plugin_wrap.ts) |  
| **`wrap://` Resolution** | | | | |  
| | Resolve a wrap package | Yes | [:mag:](./specs/resolve_package.yaml) | [:heavy_check_mark:](./clients/js/src/features/resolve_package.ts) |  
| | Resolve a wrap instance | Yes | [:mag:](./specs/resolve_instance.yaml) | [:heavy_check_mark:](./clients/js/src/features/resolve_instance.ts) |  
| | Resolve a redirect | Yes | [:mag:](./specs/resolve_redirect.yaml) | [:heavy_check_mark:](./clients/js/src/features/resolve_redirect.ts) |  
| | Resolve `wrap://http/` &<br/>`wrap://https/` | Yes | [:mag:](./specs/resolve_http.yaml) | [:heavy_check_mark:](./clients/js/src/features/resolve_http.ts) |  
| | Resolve `wrap://file/` &<br/>`wrap://fs/` | Yes | [:mag:](./specs/resolve_file.yaml) | [:heavy_check_mark:](./clients/js/src/features/resolve_file.ts) |  
| | Resolve `wrap://ipfs/` | Yes | [:mag:](./specs/resolve_ipfs.yaml) | [:heavy_check_mark:](./clients/js/src/features/resolve_ipfs.ts) |  
| | Resolve `wrap://ens/` contenthash | No | [:mag:](./specs/resolve_ens_contenthash.yaml) | [:heavy_check_mark:](./clients/js/src/features/resolve_ens_contenthash.ts) |  
| | Resolve `wrap://ens/` text-records | No | [:mag:](./specs/resolve_ens_text_record.yaml) | [:heavy_check_mark:](./clients/js/src/features/resolve_ens_text_record.ts) |  
| **[WRAP Test Harness](https://github.com/polywrap/wrap-test-harness/tree/master/cases)** | | | | |  
| | `env-type` test wrap | Yes | [:mag:](./specs/wrap_test_harness_env_type.yaml) | [:heavy_check_mark:](./clients/js/src/features/wrap_test_harness_env_type.ts) |  
| | `interface-invoke` test wrap | Yes | [:mag:](./specs/wrap_test_harness_interface_invoke.yaml) | [:heavy_check_mark:](./clients/js/src/features/wrap_test_harness_interface_invoke.ts) |  
| | `bigint-type` test wrap | Yes | [:mag:](./specs/wrap_test_harness_bigint_type.yaml) | [:heavy_check_mark:](./clients/js/src/features/wrap_test_harness_bigint_type.ts) |  
| | `bignumber-type` test wrap | Yes | [:mag:](./specs/wrap_test_harness_bignumber_type.yaml) | [:heavy_check_mark:](./clients/js/src/features/wrap_test_harness_bignumber_type.ts) |  
| | `map-type` test wrap | Yes | [:mag:](./specs/wrap_test_harness_map_type.yaml) | [:heavy_check_mark:](./clients/js/src/features/wrap_test_harness_map_type.ts) |  
