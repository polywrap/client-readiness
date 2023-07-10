# Client Readiness Checklist
A place to define a "readiness checklist" for all Polywrap clients to adhere to.

## Checklist

| Feature :heavy_check_mark: | Scenario :thought_balloon: | Required :question: | Spec :clipboard: | JS :scroll: | KT :scroll: | Swift :eagle: |
|-|-|-|-|-|-|-|
| **`wrap://` URIs** | | | | | | |
| | Create & sanitize a `wrap://` URI | Yes | [:mag:](./specs/uri.yaml) | [:heavy_check_mark:](./clients/js/src/features/uri.ts) | [:heavy_check_mark:](./clients/kotlin/src/main/kotlin/features/uri/uri.kt) | [:heavy_check_mark:](./clients/swift/Sources/Readiness/Features/Uri.swift) |
| **Client Configuration** | | | | | | |
| | Add a wrap package | Yes | [:mag:](./specs/config_embed_wrap_package.yaml) | [:heavy_check_mark:](./clients/js/src/features/config_embed_wrap_package.ts) | [:heavy_check_mark:](./clients/kotlin/src/main/kotlin/features/config/EmbedWrapPackage.kt) | [:heavy_check_mark:](./clients/swift/Sources/Readiness/Features/ConfigEmbedWrapPackage.swift) |
| | Add a plugin package | Yes | [:mag:](./specs/config_plugin_package.yaml) | [:heavy_check_mark:](./clients/js/src/features/config_plugin_package.ts) | [:heavy_check_mark:](./clients/kotlin/src/main/kotlin/features/config/PluginPackage.kt) | [:heavy_check_mark:](./clients/swift/Sources/Readiness/Features/ConfigPluginPackage.swift) |
| | Add a plugin instance | Yes | [:mag:](./specs/config_plugin_instance.yaml) | [:heavy_check_mark:](./clients/js/src/features/config_plugin_instance.ts) | [:heavy_check_mark:](./clients/kotlin/src/main/kotlin/features/config/PluginInstance.kt) | [:heavy_check_mark:](./clients/swift/Sources/Readiness/Features/ConfigPluginInstance.swift) |
| | Add a URI redirect | Yes | [:mag:](./specs/config_uri_redirect.yaml) | [:heavy_check_mark:](./clients/js/src/features/config_uri_redirect.ts) | [:x:](./clients/kotlin/src/main/kotlin/features/config/UriRedirect.kt) | [:x:](./clients/swift/Sources/Readiness/Features/ConfigUriRedirect.swift) |
| | Add env variables | Yes | [:mag:](./specs/config_env_variables.yaml) | [:heavy_check_mark:](./clients/js/src/features/config_env_variables.ts) | [:heavy_check_mark:](./clients/kotlin/src/main/kotlin/features/config/EnvVariables.kt) | [:heavy_check_mark:](./clients/swift/Sources/Readiness/Features/ConfigEnvVariables.swift) |
| | Add interface implementations | Yes | [:mag:](./specs/config_interface_implementations.yaml) | [:heavy_check_mark:](./clients/js/src/features/config_interface_implementations.ts) | [:heavy_check_mark:](./clients/kotlin/src/main/kotlin/features/config/InterfaceImplementations.kt) | [:heavy_check_mark:](./clients/swift/Sources/Readiness/Features/ConfigInterfaceImplementations.swift) |
| | Add resolver | Yes | [:mag:](./specs/config_resolver.yaml) | [:heavy_check_mark:](./clients/js/src/features/config_resolver.ts) | [:x:](./clients/kotlin/src/main/kotlin/features/config/Resolver.kt) | [:x:](./clients/swift/Sources/Readiness/Features/ConfigResolver.swift) |
| | Add resolver ext | Yes | [:mag:](./specs/config_resolver_ext.yaml) | [:heavy_check_mark:](./clients/js/src/features/config_resolver_ext.ts) | [:x:](./clients/kotlin/src/main/kotlin/features/config/ResolverExt.kt) | [:x:](./clients/swift/Sources/Readiness/Features/ConfigResolverExt.swift) |
| **Invocation** | | | | | | |
| | Invoke a wrap function<br/>(Wasm Runtime v0.1) | Yes | [:mag:](./specs/invoke_wrap_wasm_v0_1.yaml) | [:heavy_check_mark:](./clients/js/src/features/invoke_wrap_wasm_v0_1.ts) | [:heavy_check_mark:](./clients/kotlin/src/main/kotlin/features/invoke/WrapWasmV01.kt) | [:heavy_check_mark:](./clients/swift/Sources/Readiness/Features/InvokeWrapWasmV0_1.swift) |
| | Invoke a plugin function | Yes | [:mag:](./specs/invoke_plugin.yaml) | [:heavy_check_mark:](./clients/js/src/features/invoke_plugin.ts) | [:heavy_check_mark:](./clients/kotlin/src/main/kotlin/features/invoke/Plugin.kt) | [:heavy_check_mark:](./clients/swift/Sources/Readiness/Features/InvokePlugin.swift) |
| | Subinvoke: `wrap -> wrap` | Yes | [:mag:](./specs/subinvoke_wrap_wrap.yaml) | [:heavy_check_mark:](./clients/js/src/features/subinvoke_wrap_wrap.ts) | [:heavy_check_mark:](./clients/kotlin/src/main/kotlin/features/subinvoke/WrapWrap.kt) | [:heavy_check_mark:](./clients/swift/Sources/Readiness/Features/SubinvokeWrapWrap.swift) |
| | Subinvoke: `wrap -> plugin` | Yes | [:mag:](./specs/subinvoke_wrap_plugin.yaml) | [:heavy_check_mark:](./clients/js/src/features/subinvoke_wrap_plugin.ts) | [:heavy_check_mark:](./clients/kotlin/src/main/kotlin/features/subinvoke/WrapPlugin.kt) | [:heavy_check_mark:](./clients/swift/Sources/Readiness/Features/SubinvokeWrapPlugin.swift) |
| | Subinvoke: `plugin -> wrap` | Yes | [:mag:](./specs/subinvoke_plugin_wrap.yaml) | [:heavy_check_mark:](./clients/js/src/features/subinvoke_plugin_wrap.ts) | [:heavy_check_mark:](./clients/kotlin/src/main/kotlin/features/subinvoke/PluginWrap.kt) | [:heavy_check_mark:](./clients/swift/Sources/Readiness/Features/SubinvokePluginWrap.swift) |
| **`wrap://` Resolution** | | | | |                                                                                                    |
| | Resolve a wrap package | Yes | [:mag:](./specs/resolve_package.yaml) | [:heavy_check_mark:](./clients/js/src/features/resolve_package.ts) | [:x:](./clients/kotlin/src/main/kotlin/features/resolve/Package.kt) | :x: |
| | Resolve a wrap instance | Yes | [:mag:](./specs/resolve_instance.yaml) | [:heavy_check_mark:](./clients/js/src/features/resolve_instance.ts) | [:x:](./clients/kotlin/src/main/kotlin/features/resolve/Instance.kt) | :x: |
| | Resolve a redirect | Yes | [:mag:](./specs/resolve_redirect.yaml) | [:heavy_check_mark:](./clients/js/src/features/resolve_redirect.ts) | [:x:](./clients/kotlin/src/main/kotlin/features/resolve/Redirect.kt) | :x: |
| | Resolve `wrap://http/` &<br/>`wrap://https/` | Yes | [:mag:](./specs/resolve_http.yaml) | [:heavy_check_mark:](./clients/js/src/features/resolve_http.ts) | [:x:](./clients/kotlin/src/main/kotlin/features/resolve/Http.kt) | :x: |
| | Resolve `wrap://file/` &<br/>`wrap://fs/` | Yes | [:mag:](./specs/resolve_file.yaml) | [:heavy_check_mark:](./clients/js/src/features/resolve_file.ts) | [:x:](./clients/kotlin/src/main/kotlin/features/resolve/File.kt) | :x: |
| | Resolve `wrap://ipfs/` | Yes | [:mag:](./specs/resolve_ipfs.yaml) | [:heavy_check_mark:](./clients/js/src/features/resolve_ipfs.ts) | [:x:](./clients/kotlin/src/main/kotlin/features/resolve/Ipfs.kt) | :x: |
| | Resolve `wrap://ens/` contenthash | No | [:mag:](./specs/resolve_ens_contenthash.yaml) | [:heavy_check_mark:](./clients/js/src/features/resolve_ens_contenthash.ts) | [:x:](./clients/kotlin/src/main/kotlin/features/resolve/EnsContentHash.kt) | :x: |
| | Resolve `wrap://ens/` text-records | No | [:mag:](./specs/resolve_ens_text_record.yaml) | [:heavy_check_mark:](./clients/js/src/features/resolve_ens_text_record.ts) | [:x:](./clients/kotlin/src/main/kotlin/features/resolve/EnsTextRecord.kt) | :x: |
| **[WRAP Features](https://github.com/polywrap/wrap-test-harness/tree/master/cases)** | | | | | | |
| | Wrap `Env` Variables | Yes | [:mag:](./specs/wrap_feature_env_vars.yaml) | [:heavy_check_mark:](./clients/js/src/features/wrap_feature_env_vars.ts) | [:heavy_check_mark:](./clients/kotlin/src/main/kotlin/features/wrapFeature/EnvVars.kt) | [:heavy_check_mark:](./clients/swift/Sources/Readiness/Features/WrapFeatureEnvVars.swift) |
| | Wrap Interface Invocations | Yes | [:mag:](./specs/wrap_feature_interface_invoke.yaml) | [:heavy_check_mark:](./clients/js/src/features/wrap_feature_interface_invoke.ts) | [:heavy_check_mark:](./clients/kotlin/src/main/kotlin/features/wrapFeature/InterfaceInvoke.kt) | [:heavy_check_mark:](./clients/swift/Sources/Readiness/Features/WrapFeatureInterfaceInvoke.swift) |
| **[WRAP ABI Type Support](https://github.com/polywrap/wrap-test-harness/tree/master/cases)** | | | | | | |
| | `(U)Int(8\|16\|32)` | Yes | [:mag:](./specs/wrap_type_ints.yaml) | [:heavy_check_mark:](./clients/js/src/features/wrap_type_ints.ts) | [:heavy_check_mark:](./clients/kotlin/src/main/kotlin/features/wrapType/Ints.kt) | [:heavy_check_mark:](./clients/swift/Sources/Readiness/Features/WrapTypeInts.swift) |
| | `String` | Yes | TODO | TODO | TODO | TODO |
| | `Bytes` | Yes | [:mag:](./specs/wrap_type_bytes.yaml) | [:heavy_check_mark:](./clients/js/src/features/wrap_type_bytes.ts) | [:heavy_check_mark:](./clients/kotlin/src/main/kotlin/features/wrapType/Bytes.kt) | [:heavy_check_mark:](./clients/swift/Sources/Readiness/Features/WrapTypeBytes.swift) |
| | `Object` | Yes | [:mag:](./specs/wrap_type_object.yaml) | [:heavy_check_mark:](./clients/js/src/features/wrap_type_object.ts) | [:heavy_check_mark:](./clients/kotlin/src/main/kotlin/features/wrapType/Object.kt) | [:heavy_check_mark:](./clients/swift/Sources/Readiness/Features/WrapTypeObject.swift) |
| | `Enum` | Yes | [:mag:](./specs/wrap_type_enum.yaml) | [:heavy_check_mark:](./clients/js/src/features/wrap_type_enum.ts) | [:heavy_check_mark:](./clients/kotlin/src/main/kotlin/features/wrapType/Enum.kt) | [:heavy_check_mark:](./clients/swift/Sources/Readiness/Features/WrapTypeEnum.swift) |
| | `BigInt` | Yes | [:mag:](./specs/wrap_type_bigint.yaml) | [:heavy_check_mark:](./clients/js/src/features/wrap_type_bigint.ts) | [:heavy_check_mark:](./clients/kotlin/src/main/kotlin/features/wrapType/BigInt.kt) | [:heavy_check_mark:](./clients/swift/Sources/Readiness/Features/WrapTypeBigint.swift) |
| | `BigNumber` | Yes | [:mag:](./specs/wrap_type_bignumber.yaml) | [:heavy_check_mark:](./clients/js/src/features/wrap_type_bignumber.ts) | [:heavy_check_mark:](./clients/kotlin/src/main/kotlin/features/wrapType/BigNumber.kt) | [:heavy_check_mark:](./clients/swift/Sources/Readiness/Features/WrapTypeBignumber.swift) |
| | `JSON` | Yes | [:mag:](./specs/wrap_type_json.yaml) | [:heavy_check_mark:](./clients/js/src/features/wrap_type_json.ts) | [:heavy_check_mark:](./clients/kotlin/src/main/kotlin/features/wrapType/Json.kt) | [:heavy_check_mark:](./clients/swift/Sources/Readiness/Features/WrapTypeJson.swift) |
| | `Array<T>` | Yes | TODO | TODO | TODO | TODO |
| | `Map<K, V>` | Yes | [:mag:](./specs/wrap_type_map.yaml) | [:heavy_check_mark:](./clients/js/src/features/wrap_type_map.ts) | [:heavy_check_mark:](./clients/kotlin/src/main/kotlin/features/wrapType/Map.kt) | [:heavy_check_mark:](./clients/swift/Sources/Readiness/Features/WrapTypeMap.swift) |


## Build & Contribute

### Prerequisites

- [NVM](https://github.com/nvm-sh/nvm)
- [Curl](https://curl.se/)
- [Yarn](https://yarnpkg.com/)

### Install

```bash
./install.sh
```

### Usage

```bash
./run.sh [implementation] [feature]
```

- implementation (e.g: js|rs|py)
- feature (e.g: resolve_http|uri) -> check [specs](./specs) for all features