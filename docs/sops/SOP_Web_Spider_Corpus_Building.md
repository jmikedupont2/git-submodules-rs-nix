## SOP: Web Spider for Corpus Building

### 1. Purpose
This Standard Operating Procedure (SOP) outlines the process for using the `web_spider_corpus_builder` tool to fetch web content from specified URLs and organize it into a local corpus. This tool is essential for gathering external textual data relevant to conceptual discussions, research, or analysis.

### 2. Tool Overview: `web_spider_corpus_builder`

#### 2.1. Location
The `web_spider_corpus_builder` tool is located in `tools/web_spider_corpus_builder/`.

#### 2.2. Functionality
The tool performs the following actions:
*   Reads URLs from specified Markdown files (via command-line arguments).
*   For each URL, it attempts to fetch the web page content.
*   It prioritizes `text/html` content, extracting text from paragraph (`<p>`) tags.
*   It handles `application/pdf` by noting that PDFs are skipped (as direct text extraction is complex).
*   Other unsupported content types are also noted as skipped.
*   Extracted text content is saved to *new*, individual `.txt` files within a designated output directory. Existing files are never modified or appended to.
*   Basic rate limiting (1-second delay between requests) is implemented to be polite to web servers.

### 3. Usage

#### 3.1. Building the Tool
Navigate to the project root directory and build the tool using Cargo:
```bash
cargo build -p web_spider_corpus_builder
```

#### 3.2. Running the Tool
After successful compilation, run the executable from the project root, providing the paths to the Markdown files containing the URLs:
```bash
target/debug/web_spider_corpus_builder -m docs/reflection/conceptual_path_reconstruction/006_intrinsic_properties_of_5.md docs/reflection/conceptual_path_reconstruction/007_self_referential_numbers.md
```

#### 3.3. Output
The tool will create a directory named `corpus/web_sources/` in the project root. Inside this directory, each successfully fetched and processed web page will have its extracted text content saved as a *new* `.txt` file. The filenames are derived from the URL to ensure uniqueness and readability. No existing files will be modified.

### 4. Configuration
URLs are now read from Markdown files specified as command-line arguments. This allows for flexible input of URLs without modifying the source code.

### 5. Error Handling
The tool includes basic error handling for network issues and file operations. Failed fetches or unsupported content types will be reported to `stderr` or `stdout`.

### 6. Testing
Integration tests for the `web_spider_corpus_builder` are located in `tools/web_spider_corpus_builder/tests/integration_test.rs`. These tests verify the creation of the corpus directory, the fetching and saving of HTML content, and the handling of different content types.
