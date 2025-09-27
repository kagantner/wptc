import yaml
import os
import argparse
import re

# --- Configuration ---
# The configuration file and output directory are now handled by command-line arguments.
OUTPUT_FILENAME = 'manuscript.rtf' # The name of the final manuscript file.

def convert_to_rtf(text):
    """
    Converts a string with simple Markdown to an RTF-formatted string.
    """
    # Escape special RTF characters
    text = text.replace('\\', '\\\\')
    text = text.replace('{', '\\{')
    text = text.replace('}', '\\}')
    
    # --- Markdown to RTF Conversions ---
    # Note: Order matters here, from most specific to least.
    # Bold and Italic (***text***)
    text = re.sub(r'\*\*\*(.*?)\*\*\*', r'{\\b\\i \1}', text, flags=re.DOTALL)
    # Bold (**text**)
    text = re.sub(r'\*\*(.*?)\*\*', r'{\\b \1}', text, flags=re.DOTALL)
    # Italic (*text* or _text_)
    text = re.sub(r'\*(.*?)\*', r'{\\i \1}', text, flags=re.DOTALL)
    text = re.sub(r'\_(.*?)\_', r'{\\i \1}', text, flags=re.DOTALL)
    
    # Convert hyphens to em-dashes
    text = text.replace('---', '\\emdash ')
    
    # Convert standard newlines to RTF paragraph breaks
    text = text.replace('\n', '\\par\n')
    
    return text

def create_title_page(config, f_out):
    """Generates and writes a standard manuscript title page in RTF format."""
    author = config.get('author', {})
    metadata = config.get('metadata', {})
    
    # Standard RTF font settings for the title page (Times New Roman)
    f_out.write('{\\f1\\fs24 ')

    # Write author contact info (top-left)
    f_out.write(convert_to_rtf(author.get('legal_name', '')))
    if 'address' in author:
        f_out.write(convert_to_rtf(f"\n{author['address']}"))
    f_out.write(convert_to_rtf(f"\n{author.get('phone', '')}"))
    f_out.write(convert_to_rtf(f"\n{author.get('email', '')}\n\n"))
    
    # Add approximate word count
    word_count = metadata.get('word_count', 0)
    f_out.write(convert_to_rtf(f"Approx. {word_count:,} words\n\n\n\n"))

    # Center the title and byline
    title = metadata.get('title', 'Untitled Novel').upper()
    byline = f"by\n{metadata.get('byline', 'Anonymous')}"
    
    # Use RTF codes for centering and bolding
    f_out.write('{\\qc ') # Center align
    f_out.write(f'{{\\b {convert_to_rtf(title)}}}') # Bold title
    f_out.write('\\par\\par\n')
    f_out.write(f'{convert_to_rtf(byline)}')
    f_out.write(' \\par}\n') # End centering

    # Add a "page break" before the content starts
    f_out.write('\\page\n')
    f_out.write('}') # Close font block

def append_file_content(filepath, f_out):
    """
    Reads markdown content from a given filepath, ignores heading lines,
    converts the rest to RTF, and appends it to the output file.
    """
    try:
        with open(filepath, 'r', encoding='utf-8') as f_in:
            lines_to_keep = []
            for line in f_in:
                if not re.match(r'^#{1,6}\s', line.lstrip()):
                    lines_to_keep.append(line)
            
            content_block = "".join(lines_to_keep).strip()
            rtf_content = convert_to_rtf(content_block)
            
            # Write the content with double-spacing, first-line indent, and standard font settings
            f_out.write('{\\f1\\fs24\\sl480\\fi720 ') # Set font, size, double-spacing, and indent
            f_out.write(rtf_content)
            f_out.write('\\par\n') # Ensure a paragraph break after the block
            f_out.write('}')

    except FileNotFoundError:
        print(f"--> WARNING: Could not find file: {filepath}. It will be skipped.")
    except Exception as e:
        print(f"--> ERROR: An error occurred reading {filepath}: {e}")

def process_chapter(chapter_data, f_out):
    """Processes a chapter dictionary and writes its content in RTF."""
    heading = []
    if 'number' in chapter_data:
        heading.append(f"Chapter {chapter_data['number']}")
    if 'title' in chapter_data:
        heading.append(chapter_data['title'])
    
    if heading:
        full_heading = ": ".join(heading)
        # Write heading as centered, bold, 12pt Times New Roman
        f_out.write('{\\f1\\fs24\\qc\\b ')
        f_out.write(convert_to_rtf(full_heading))
        f_out.write('}\\par\\par\n')

    if 'file' in chapter_data:
        append_file_content(chapter_data['file'], f_out)
    elif 'files' in chapter_data:
        file_paths = chapter_data.get('files', [])
        for i, file_path in enumerate(file_paths):
            append_file_content(file_path, f_out)
            # Add a scene break if this is NOT the last file in the list.
            if i < len(file_paths) - 1:
                f_out.write('{\\f1\\fs24\\qc # \\par}\n') # Centered # for scene break

def compile_manuscript(config_file, output_dir):
    """Main function to read the YAML config and compile the manuscript into an RTF file."""
    print(f"Starting manuscript compilation from '{config_file}'...")

    try:
        with open(config_file, 'r', encoding='utf-8') as f:
            config = yaml.safe_load(f)
    except FileNotFoundError:
        print(f"--> FATAL ERROR: The configuration file '{config_file}' was not found.")
        return
    except yaml.YAMLError as e:
        print(f"--> FATAL ERROR: There was an error parsing the YAML file: {e}")
        return

    if not os.path.exists(output_dir):
        print(f"Creating output directory: '{output_dir}'")
        os.makedirs(output_dir)
        
    output_path = os.path.join(output_dir, OUTPUT_FILENAME)

    with open(output_path, 'w', encoding='utf-8') as f_out:
        # 1. Write RTF Header
        f_out.write('{\\rtf1\\ansi\\deff0\\nouicompat\n')
        # Define Times New Roman as our primary font (f1)
        f_out.write('{\\fonttbl{\\f1\\fnil\\fcharset0 Times New Roman;}}\n')
        # Set 1-inch margins (1 inch = 1440 twips)
        f_out.write('\\margl1440\\margr1440\\margt1440\\margb1440\n')

        # 2. Create the title page
        create_title_page(config, f_out)

        is_first_content_item = True

        # 3. Process the main structure
        for item in config.get('structure', []):
            item_type = item.get('type')

            # Add a page break before any top-level part or chapter,
            # except for the very first one after the title page.
            if not is_first_content_item:
                f_out.write('\\page\n')
            is_first_content_item = False
            
            if item_type == 'part':
                part_title = item.get('title', 'Untitled Part').upper()
                # The page break for the part title was already handled above.
                f_out.write('{\\f1\\fs24\\qc\\b ') # Centered, bold
                f_out.write(convert_to_rtf(part_title))
                f_out.write('}\\par\\par\\par\n')
                
                # Process chapters within the part.
                chapters_in_part = item.get('content', [])
                for i, chapter in enumerate(chapters_in_part):
                    # Add a page break ONLY if it's not the first chapter of the part.
                    if i > 0:
                        f_out.write('\\page\n')
                    process_chapter(chapter, f_out)
            
            elif item_type == 'chapter':
                # The page break for standalone chapters was handled above.
                process_chapter(item, f_out)
            
            else:
                print(f"--> WARNING: Unknown structure type '{item_type}' found. Skipping.")
        
        # Add a final centered end mark on a new page.
        f_out.write('\\page\n')
        f_out.write('{\\f1\\fs24\\qc #  #  #\\par}\n')
        
        # 4. Write RTF Footer
        f_out.write('}')

    print(f"\nCompilation complete! Manuscript saved to '{output_path}'.")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Compiles a novel manuscript from a YAML configuration file into a single RTF file."
    )
    
    parser.add_argument(
        "config_file", 
        help="Path to the YAML configuration file (e.g., testbuild.yaml)."
    )
    
    parser.add_argument(
        "output_dir", 
        help="The directory where the compiled manuscript file will be saved (e.g., build/)."
    )
    
    args = parser.parse_args()
    compile_manuscript(args.config_file, args.output_dir)

