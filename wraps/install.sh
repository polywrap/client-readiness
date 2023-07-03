# Download test wrappers
curl -L https://github.com/polywrap/wrap-test-harness/releases/download/0.1.1/wrappers --output wrappers.zip
unzip -o wrappers.zip
rm -rf wrappers.zip

# Publish one wrapper to a public directory (used for http resolution tests)
rm -rf public
mkdir public
cp ./numbers-type/implementations/as/wrap.info ./public/wrap.info
cp ./numbers-type/implementations/as/wrap.wasm ./public/wrap.wasm
