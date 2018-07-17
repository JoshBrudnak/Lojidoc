cargo rustdoc -- --document-private-items -v
cp -r ./target/doc/* ./docs/

echo "Documentation generated."
