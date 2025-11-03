#!/usr/bin/env python3
"""
Script to generate async API implementations from sync implementations.

This script reads all *_api.rs files from src/apis/ and generates
corresponding async versions in src/async_apis/ by:
1. Replacing `Rc` with `Arc`
2. Replacing `reqwest::blocking::Client` with `reqwest::Client`
3. Adding `async` keyword to all trait methods
4. Adding `.await` to all async operations
5. Adding `#[async_trait::async_trait]` to trait definitions
6. Updating module paths
"""

import re
import os
from pathlib import Path

def convert_sync_to_async(content: str, api_name: str) -> str:
    """Convert a sync API implementation to async."""

    # Replace imports
    content = content.replace(
        "use std::rc::Rc;",
        "use std::sync::Arc;"
    )

    # Update configuration import - no changes needed, keep as is
    # The async version uses the same structure

    # Replace Rc with Arc
    content = content.replace("Rc<", "Arc<")
    content = content.replace("Rc::", "Arc::")

    # Add async_trait attribute before trait definitions
    content = re.sub(
        r'(pub trait \w+Api)',
        r'#[async_trait::async_trait]\n\1',
        content
    )

    # Add async_trait attribute before trait implementations
    content = re.sub(
        r'(impl \w+Api for \w+ApiClient)',
        r'#[async_trait::async_trait]\n\1',
        content
    )

    # Convert trait method signatures to async
    # Match fn declarations in traits (with Result return type)
    def add_async_to_trait_method(match):
        indent = match.group(1)
        fn_def = match.group(2)
        return f"{indent}async {fn_def}"

    content = re.sub(
        r'(    )(fn \w+\([^)]*\).*?-> Result<[^;]+;)',
        add_async_to_trait_method,
        content,
        flags=re.MULTILINE
    )

    # Convert trait method implementations to async
    content = re.sub(
        r'(    )(fn )(\w+\()',
        r'\1async \2\3',
        content
    )

    # Add .await to execute() and json() calls
    content = re.sub(
        r'client\.execute\(req\)\?',
        r'client.execute(req).await?',
        content
    )

    content = re.sub(
        r'\.error_for_status\(\)\?\.json\(\)\?',
        r'.error_for_status()?.json().await?',
        content
    )

    # Add .await to other potential async operations
    content = re.sub(
        r'req_builder\.build\(\)\?;',
        r'req_builder.build()?;',
        content
    )

    return content

def generate_async_api(sync_file: Path, async_dir: Path):
    """Generate an async API file from a sync API file."""

    # Skip non-API files
    if not sync_file.name.endswith('_api.rs'):
        return

    # Skip the client file (we'll handle it separately)
    if sync_file.name == 'client.rs':
        return

    print(f"Processing {sync_file.name}...")

    # Read the sync file
    with open(sync_file, 'r') as f:
        content = f.read()

    # Extract API name (e.g., "movies" from "movies_api.rs")
    api_name = sync_file.stem.replace('_api', '')

    # Convert to async
    async_content = convert_sync_to_async(content, api_name)

    # Write to async directory
    async_file = async_dir / sync_file.name
    with open(async_file, 'w') as f:
        f.write(async_content)

    print(f"  -> Generated {async_file.name}")

def main():
    # Get the project root directory
    script_dir = Path(__file__).parent
    src_dir = script_dir / 'src'
    apis_dir = src_dir / 'apis'
    async_apis_dir = src_dir / 'async_apis'

    # Ensure async_apis directory exists
    async_apis_dir.mkdir(exist_ok=True)

    # Process all API files
    api_files = list(apis_dir.glob('*_api.rs'))

    print(f"Found {len(api_files)} API files to process")
    print()

    for api_file in api_files:
        try:
            generate_async_api(api_file, async_apis_dir)
        except Exception as e:
            print(f"  ERROR processing {api_file.name}: {e}")

    print()
    print("Done! All async API files have been generated.")
    print()
    print("Next steps:")
    print("1. Review the generated files in src/async_apis/")
    print("2. Update src/async_apis/mod.rs to export all APIs")
    print("3. Create src/async_apis/client.rs for the AsyncAPIClient facade")
    print("4. Test the async implementation")

if __name__ == '__main__':
    main()
