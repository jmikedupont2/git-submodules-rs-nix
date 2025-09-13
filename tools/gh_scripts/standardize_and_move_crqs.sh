#!/usr/bin/env bash

DRY_RUN=false
if [[ "$1" == "--dry-run" ]]; then
  DRY_RUN=true
  echo "Running in DRY-RUN mode. No files will be modified."
fi

CRQ_DIR="/data/data/com.termux.nix/files/home/pick-up-nix/source/github/meta-introspector/submodules/docs/crq/"
STANDARDIZED_DIR="/data/data/com.termux.nix/files/home/pick-up-nix/source/github/meta-introspector/submodules/docs/crq_standardized/"

# Ensure standardized directory exists
if [ ! -d "$STANDARDIZED_DIR" ]; then
  if $DRY_RUN;
  then
    echo "Would create directory: $STANDARDIZED_DIR"
  else
    mkdir -p "$STANDARDIZED_DIR"
  fi
fi

CRQ_FILES=$(find "$CRQ_DIR" -maxdepth 1 -name "*.md" -print | sort)

PROCESSED_COUNT=0
STANDARDIZED_COUNT=0
FAILED_COUNT=0

# --- Start: Robust HIGHEST_CRQ_NUMBER calculation ---
HIGHEST_CRQ_NUMBER=0

# Collect all potential CRQ numbers from filenames in both directories
ALL_CRQ_NUMBERS=()
for file in $(find "$CRQ_DIR" "$STANDARDIZED_DIR" -maxdepth 1 -name "*.md" -print);
do
  num=$(basename "$file" | grep -oP 'CRQ-\K[0-9]+')
  if [[ -n "$num" ]]; then
    ALL_CRQ_NUMBERS+=("$num")
  fi
done

# Iterate through all files to find the highest CRQ number from their content if they have a CRQ-XXX header
for CRQ_FILE_PATH in $CRQ_FILES;
do
  CRQ_NUMBER_FROM_HEADER=$(grep -m 1 "^# CRQ-\\K[0-9]+" "$CRQ_FILE_PATH" | grep -oP 'CRQ-\K[0-9]+')
  if [[ -n "$CRQ_NUMBER_FROM_HEADER" ]]; then
    ALL_CRQ_NUMBERS+=("$CRQ_NUMBER_FROM_HEADER")
  fi
done

# Find the maximum number from the collected list
for num in "${ALL_CRQ_NUMBERS[@]}"; do
  if (( 10#$num > 10#$HIGHEST_CRQ_NUMBER )); then # Force base-10 comparison
    HIGHEST_CRQ_NUMBER="$num"
  fi
done

NEXT_CRQ_NUMBER=$((10#$HIGHEST_CRQ_NUMBER + 1)) # Force base-10 addition
# --- End: Robust HIGHEST_CRQ_NUMBER calculation ---

for CRQ_FILE_PATH in $CRQ_FILES;
do
  PROCESSED_COUNT=$((PROCESSED_COUNT + 1))
  FILENAME=$(basename "$CRQ_FILE_PATH")
  
  echo "\n--- Processing $FILENAME ---"

  # Extract CRQ Number (more robustly)
  CRQ_NUMBER=$(echo "$FILENAME" | grep -oP 'CRQ-\K[0-9]+' || echo "")
  
  # Extract CRQ Title from content (more robustly)
  CRQ_TITLE_FROM_HEADER=$(grep -m 1 "^## Change Request:" "$CRQ_FILE_PATH" | sed 's/## Change Request: //g' | tr -d '\r')
  if [ -z "$CRQ_TITLE_FROM_HEADER" ]; then
    CRQ_TITLE_FROM_HEADER=$(grep -m 1 "^# " "$CRQ_FILE_PATH" | sed 's/^# //g' | tr -d '\r')
  fi

  # Determine final title and number
  FINAL_CRQ_NUMBER=""
  FINAL_CRQ_TITLE=""
  NEEDS_RENAME=false
  NEEDS_HEADER_UPDATE=false

  if [[ -n "$CRQ_NUMBER" && -n "$CRQ_TITLE_FROM_HEADER" ]]; then
    # Conforming filename and header
    FINAL_CRQ_NUMBER="$CRQ_NUMBER"
    FINAL_CRQ_TITLE="$CRQ_TITLE_FROM_HEADER"
    echo "Status: Conforming filename and header."
  elif [[ -n "$CRQ_NUMBER" && -z "$CRQ_TITLE_FROM_HEADER" ]]; then
    # Conforming filename, but missing/non-conforming header
    FINAL_CRQ_NUMBER="$CRQ_NUMBER"
    FINAL_CRQ_TITLE=$(echo "$FILENAME" | sed 's/CRQ-[0-9]*-//g' | sed 's/\.md$//g' | sed 's/[_-]/ /g')
    NEEDS_HEADER_UPDATE=true
    echo "Status: Conforming filename, but header needs update."
  elif [[ -z "$CRQ_NUMBER" && -n "$CRQ_TITLE_FROM_HEADER" ]]; then
    # Non-conforming filename, but conforming header
    FINAL_CRQ_NUMBER="$NEXT_CRQ_NUMBER"
    FINAL_CRQ_TITLE="$CRQ_TITLE_FROM_HEADER"
    NEEDS_RENAME=true
    NEEDS_HEADER_UPDATE=true # Set to true if renamed, to update header with new CRQ number
    NEXT_CRQ_NUMBER=$((10#$NEXT_CRQ_NUMBER + 1)) # Increment for next assignment
    echo "Status: Non-conforming filename, but header is good. Assigning new CRQ number."
  else
    # Completely non-conforming
    FINAL_CRQ_NUMBER="$NEXT_CRQ_NUMBER"
    FINAL_CRQ_TITLE=$(echo "$FILENAME" | sed 's/\.md$//g' | sed 's/[_-]/ /g')
    NEEDS_RENAME=true
    NEEDS_HEADER_UPDATE=true
    NEXT_CRQ_NUMBER=$((10#$NEXT_CRQ_NUMBER + 1)) # Increment for next assignment
    echo "Status: Completely non-conforming. Assigning new CRQ number and updating header."
  fi

  # Sanitize title for new filename
  SANITIZED_TITLE=$(echo "$FINAL_CRQ_TITLE" | sed 's/[^a-zA-Z0-9_ -]/ /g' | tr ' ' '-' | tr -s '-' | tr '[:upper:]' '[:lower:]')
  NEW_FILENAME="CRQ-${FINAL_CRQ_NUMBER}-${SANITIZED_TITLE}.md"
  NEW_FILE_PATH="${CRQ_DIR}${NEW_FILENAME}"
  DEST_FILE_PATH="${STANDARDIZED_DIR}${NEW_FILENAME}"

  if $DRY_RUN;
  then
    echo "  Proposed New Filename: $NEW_FILENAME"
    if $NEEDS_HEADER_UPDATE;
    then
      echo "  Proposed New Header: # CRQ-${FINAL_CRQ_NUMBER}-${SANITIZED_TITLE}.md\n\n## Change Request: ${FINAL_CRQ_TITLE}"
    fi
    echo "  Would move from $CRQ_FILE_PATH to $DEST_FILE_PATH"
  else
    # Perform header update if needed
    if $NEEDS_HEADER_UPDATE;
    then
      echo "Updating header for $FILENAME..."
      # Read content, prepend new header, write back
      CURRENT_CONTENT=$(cat "$CRQ_FILE_PATH")
      NEW_HEADER_BLOCK="# CRQ-${FINAL_CRQ_NUMBER}-${SANITIZED_TITLE}.md\n\n## Change Request: ${FINAL_CRQ_TITLE}"
      # Use sed to insert at the beginning, handling potential existing first line
      echo -e "$NEW_HEADER_BLOCK\n$CURRENT_CONTENT" > "$CRQ_FILE_PATH.tmp"
      mv "$CRQ_FILE_PATH.tmp" "$CRQ_FILE_PATH"
    fi

    # Rename and move the file
    echo "Moving $FILENAME to $STANDARDIZED_DIR..."
    mv "$CRQ_FILE_PATH" "$DEST_FILE_PATH"
    STANDARDIZED_COUNT=$((STANDARDIZED_COUNT + 1))
  fi
done

echo "\n--- Standardization Report ---"
echo "Total files processed: $PROCESSED_COUNT"
echo "Files standardized and moved: $STANDARDIZED_COUNT"
echo "Files failed (should not happen with robust parsing): $FAILED_COUNT"

if $DRY_RUN;
then
  echo "(DRY-RUN complete. No files were modified.)"
fi
