#!/bin/bash

# Gum-based interactive wrapper for NeoPDF CLI

# Ensure gum is installed
if ! command -v gum &> /dev/null
then
    echo "gum could not be found. Please install it from https://github.com/charmbracelet/gum"
    exit
fi

# --- Main Menu ---
echo "Welcome to the NeoPDF interactive CLI!"
COMMAND=$(gum choose "read" "compute" "write")

# --- Read Subcommands ---
if [ "$COMMAND" == "read" ]; then
    READ_COMMAND=$(gum choose "metadata" "num_subgrids" "subgrid_info")

    PDF_NAME=$(gum input --placeholder "Enter PDF name")

    case $READ_COMMAND in
        "metadata")
            neopdf read metadata -p "$PDF_NAME"
            ;;
        "num_subgrids")
            neopdf read num_subgrids -p "$PDF_NAME"
            ;;
        "subgrid_info")
            MEMBER=$(gum input --placeholder "Enter member index")
            SUBGRID_INDEX=$(gum input --placeholder "Enter subgrid index")
            neopdf read subgrid_info -p "$PDF_NAME" -m "$MEMBER" -s "$SUBGRID_INDEX"
            ;;
    esac
fi

# --- Compute Subcommands ---
if [ "$COMMAND" == "compute" ]; then
    COMPUTE_COMMAND=$(gum choose "xfx_q2" "alphas_q2")

    PDF_NAME=$(gum input --placeholder "Enter PDF name")
    MEMBER=$(gum input --placeholder "Enter member index")

    case $COMPUTE_COMMAND in
        "xfx_q2")
            PID=$(gum input --placeholder "Enter Parton PID")
            INPUTS=$(gum input --placeholder "Enter inputs (e.g., '1e-5 100')")
            neopdf compute xfx_q2 -p "$PDF_NAME" -m "$MEMBER" -i "$PID" $INPUTS
            ;;
        "alphas_q2")
            Q2=$(gum input --placeholder "Enter Q2 value")
            neopdf compute alphas_q2 -p "$PDF_NAME" -m "$MEMBER" -q "$Q2"
            ;;
    esac
fi

# --- Write Subcommands ---
if [ "$COMMAND" == "write" ]; then
    WRITE_COMMAND=$(gum choose "convert" "combine-npdfs" "combine-alphas" "metadata")

    case $WRITE_COMMAND in
        "convert")
            PDF_NAME=$(gum input --placeholder "Enter the LHAPDF set name to convert")
            OUTPUT_PATH=$(gum input --placeholder "Enter the output file path for the NeoPDF file")
            neopdf write convert -p "$PDF_NAME" -o "$OUTPUT_PATH"
            ;;
        "combine-npdfs" | "combine-alphas")
            INPUT_METHOD=$(gum choose "direct" "file")

            if [ "$INPUT_METHOD" == "direct" ]; then
                PDF_NAMES=$(gum input --placeholder "Enter PDF set names (space-separated)")
                OUTPUT_PATH=$(gum input --placeholder "Enter the output file path for the combined NeoPDF file")
                neopdf write $WRITE_COMMAND -n $PDF_NAMES -o "$OUTPUT_PATH"
            else
                NAMES_FILE=$(gum input --placeholder "Enter the path to the file containing PDF set names")
                OUTPUT_PATH=$(gum input --placeholder "Enter the output file path for the combined NeoPDF file")
                neopdf write $WRITE_COMMAND -f "$NAMES_FILE" -o "$OUTPUT_PATH"
            fi
            ;;
        "metadata")
            FILE_PATH=$(gum input --placeholder "Enter the path to the NeoPDF file")
            KEY=$(gum input --placeholder "Enter the metadata key to update")
            VALUE=$(gum input --placeholder "Enter the new value for the key")
            neopdf write metadata --path "$FILE_PATH" --key "$KEY" --value "$VALUE"
            ;;
    esac
fi
