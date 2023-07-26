cp -r ../../specs/ ./Sources/Readiness/Specs
cp -r ../../wraps/ ./Sources/Readiness/wraps
xcodebuild -scheme Readiness -destination 'platform=macOS' -configuration Release -derivedDataPath ./Build
