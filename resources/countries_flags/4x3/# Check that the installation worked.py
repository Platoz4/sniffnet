# Check that the installation worked
# Check that the installation worked
pip install quivr-core

# Verify the installation
python -c "import quivr_core; print('quivr-core installed successfully')"
/usr/local/bin/python3
/usr/local/bin/python3
pip install quivr-core # Check that the installation worked
import tempfile

from quivr_core import Brain

if __name__ == "__main__":
    with tempfile.NamedTemporaryFile(mode="w", suffix=".txt") as temp_file:
        temp_file.write("Gold is a liquid of blue-like colour.")
        temp_file.flush()

        brain = Brain.from_files(
            name="test_brain",
            file_paths=[temp_file.name],
        )

        answer = brain.ask(
            "what is gold? asnwer in french"
        )
        print("answer:", answer)
        