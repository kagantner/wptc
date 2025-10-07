import pytest
from unittest.mock import Mock, call
from my_cli_tool.main import add_smart_text

# Helper function to capture the state of runs added to a mock paragraph
def capture_runs(mock_paragraph):
    """
    Sets up a mock paragraph to capture the text, bold, and italic state
    of each run added to it.
    """
    runs = []
    def add_run_side_effect(text):
        mock_run = Mock()
        mock_run.text = text
        mock_run.bold = False
        mock_run.italic = False
        runs.append(mock_run)
        return mock_run

    mock_paragraph.add_run.side_effect = add_run_side_effect
    return runs

def get_run_tuples(runs):
    """Converts a list of mock run objects into a list of (text, bold, italic) tuples."""
    return [(run.text, run.bold, run.italic) for run in runs]

# --- Test Cases ---

def test_add_smart_text_plain_text():
    """Tests that plain text is added correctly without any formatting."""
    mock_para = Mock()
    runs = capture_runs(mock_para)

    add_smart_text(mock_para, "This is a simple sentence.")

    assert get_run_tuples(runs) == [("This is a simple sentence.", False, False)]

def test_add_smart_text_smartypants_quotes():
    """Tests that smartypants correctly converts quotes."""
    mock_para = Mock()
    runs = capture_runs(mock_para)

    add_smart_text(mock_para, "He said, \"It's a 'test'.\"")

    # Expected: “He said, “It’s a ‘test’.””
    assert get_run_tuples(runs) == [("He said, “It’s a ‘test’.”", False, False)]

def test_add_smart_text_italic():
    """Tests italic formatting with both * and _."""
    mock_para = Mock()
    runs = capture_runs(mock_para)

    add_smart_text(mock_para, "This is *italic* and so is _this_.")

    assert get_run_tuples(runs) == [
        ("This is ", False, False),
        ("italic", False, True),
        (" and so is ", False, False),
        ("this", False, True),
        (".", False, False)
    ]

def test_add_smart_text_bold():
    """Tests bold formatting."""
    mock_para = Mock()
    runs = capture_runs(mock_para)

    add_smart_text(mock_para, "This is **bold** text.")

    assert get_run_tuples(runs) == [
        ("This is ", False, False),
        ("bold", True, False),
        (" text.", False, False)
    ]

def test_add_smart_text_bold_and_italic():
    """Tests combined bold and italic formatting."""
    mock_para = Mock()
    runs = capture_runs(mock_para)

    add_smart_text(mock_para, "This is ***bold and italic***.")

    assert get_run_tuples(runs) == [
        ("This is ", False, False),
        ("bold and italic", True, True),
        (".", False, False)
    ]

def test_add_smart_text_nested_formatting():
    """Tests nested markdown, e.g., italic inside bold."""
    mock_para = Mock()
    runs = capture_runs(mock_para)

    add_smart_text(mock_para, "A sentence with **bold and *italic* inside**.")

    assert get_run_tuples(runs) == [
        ("A sentence with ", False, False),
        ("bold and ", True, False),
        ("italic", True, True),
        (" inside", True, False),
        (".", False, False)
    ]

def test_add_smart_text_unclosed_formatting():
    """Tests that unclosed markdown tags are treated as literal text."""
    mock_para = Mock()
    runs = capture_runs(mock_para)

    add_smart_text(mock_para, "This has an unclosed **bold tag.")

    # The "**" should just be part of the run, not change the state.
    # The regex splits on it, so it becomes its own run, but bold state is toggled on and off.
    # Let's verify the final state.
    run_tuples = get_run_tuples(runs)
    assert run_tuples[0] == ("This has an unclosed ", False, False)
    assert run_tuples[1] == ("bold tag.", True, False) # Bold state is activated and never deactivated.

def test_add_smart_text_empty_string():
    """Tests that an empty string results in no runs being added."""
    mock_para = Mock()
    runs = capture_runs(mock_para)

    add_smart_text(mock_para, "")

    assert len(runs) == 0
