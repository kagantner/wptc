# Manuscript Compiler (mdmf)

A Rust CLI tool to compile a novel or long-form manuscript from a YAML configuration and individual Markdown files into a single, professionally formatted `.docx` file.

## Features

*   **YAML Configuration**: Define your manuscript's structure, metadata, author info, and file order in a single, easy-to-read YAML file.
*   **Standard Manuscript Formatting**: Automatically generates a document with Times New Roman, 12pt font, double-spaced lines, etc. Follows [William Shunn's guidelines.](https://www.shunn.net/format/)
*   **Automatic Title Page**: Creates a standard title page with author contact info and an automatically calculated approximate word count.
*   **Headers**: Adds a right-aligned header (`LastName | Short Title | PageNumber`) to every page after the title page.
*   **Chapter and Part Generation**: Correctly formats and adds Part and Chapter headings based on your manuscript's structure.
*   **Simple Markdown Support**: Supports `*italic*`, `_italic_`, `**bold**`, and `***bold-italic***` within your text files.
*   **Smart Typography**: Converts plain quotes and dashes into professional curly quotes and em-dashes.

## Installation

This tool is written in Rust and requires [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) to be installed.

To install the tool, simply run the following command in your terminal:
```bash
cargo install mdmf
```
This will download, compile, and place the `mdmf` binary in your Cargo bin path, making it available globally on your system.

## Usage

Once installed, you can run the compiler from your terminal, providing the path to your YAML configuration file and the desired output directory.

```bash
mdmf <path/to/config.yaml> <path/to/output_directory>
```

**Example:**
```bash
mdmf testbuild_novel.yaml build/
```

This command will read `testbuild_novel.yaml`, process all the specified text files listed within, and generate a single `.docx` file inside the `build/` directory with a filename specified in the configuration.

## YAML Configuration

The entire manuscript is defined by a YAML configuration file (e.g., `testbuild_novel.yaml`). This file has three main sections:

### `metadata`

Contains information for the title page and document properties.

```yaml
metadata:
  title: The Last Signal
  subtitle: A Chronicle of the Void # Optional
  byline: Jane Q. Novelist
  genre: Science Fiction
  short_title: The Last Signal      # For the page header
  last_name: Novelist               # For the page header
  file_name: test_manuscript.docx   # The name of the output file
  story_type: Novel                 # e.g., Novel, Novella, Short Story
```

### `author` & `agent`

Contact information that appears on the title page. The `agent` section is optional and will be ignored if not present.

```yaml
author:
  legal_name: Jane Quinn Novelist
  pen_name: Jane Q. Novelist
  street_address: 123 Literary Lane
  city_state_zip: New York, NY 10001
  phone: (212) 555-0123
  email: jane.q.novelist@email.com
  website: https://janeqnovelist.com

agent:
  name: John Agentman
  agency: The Best Words Literary Agency
  street_address: 456 Publishing Row
  city_state_zip: New York, NY 10002
  phone: (212) 555-0456
  email: john.agentman@bestwords.lit
```

### `structure`

This is the blueprint of your book, defining the order of parts and chapters.

*   **Parts**: Use `type: part` to create a part title page. A part contains a `content` list of chapters.
*   **Chapters**: Use `type: chapter` to define a chapter.
    *   `title` and `number` are used to generate the chapter heading (e.g., "Chapter 1: A Ship in the Dark"). Both are optional.
    *   `file`: Use for a chapter contained in a single text file.
    *   `files`: Use for a chapter composed of multiple scenes from different files. A scene break (`#`) will be automatically inserted between them.

```yaml
structure:
  # A standalone chapter (e.g., a prologue)
  - type: chapter
    title: "Prologue"
    file: "prologue.md"

  # A part containing multiple chapters
  - type: part
    title: "Part One: Echoes"
    content:
      - type: chapter
        number: 1
        title: "A Ship in the Dark"
        file: "chapter1.md"
      - type: chapter
        number: 2
        title: "Whispers on the Comms"
        files:
          - "scene1.md"
          - "scene2.md"
```
