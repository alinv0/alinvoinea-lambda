#!/bin/bash

# Function to display the release type menu and get user input
function get_release_type() {
  echo "Choose release type:"
  echo "1) Patch"
  echo "2) Minor"
  echo "3) Major"
  read -p "Enter your choice (1/2/3): " choice

  case $choice in
    1)
      echo "patch"
      ;;
    2)
      echo "minor"
      ;;
    3)
      echo "major"
      ;;
    *)
      echo "Invalid choice, defaulting to patch."
      echo "patch"
      ;;
  esac
}

# Function to display a list of subfolders containing Cargo.toml and get user input
function select_subfolder() {
  echo "Available subfolders with Cargo.toml:"
  subfolders=()
  i=1

  for dir in */ ; do
    if [[ -d "$dir" && -f "${dir}Cargo.toml" ]]; then
      echo "$i) ${dir}"
      subfolders+=("$dir")
      ((i++))
    fi
  done

  if [ ${#subfolders[@]} -eq 0 ]; then
    echo "No subfolders with Cargo.toml found."
    exit 1
  fi

  read -p "Enter the number of the subfolder you want to release: " choice
  if [[ $choice -ge 1 && $choice -le ${#subfolders[@]} ]]; then
    echo "${subfolders[$choice-1]}"
  else
    echo "Invalid choice."
    exit 1
  fi
}

# Ensure the script is run from the root directory
if [[ ! -f "release.sh" ]]; then
  echo "This script must be run from the root directory where release.sh is located."
  exit 1
fi

# Prompt the user to select a subfolder
selected_subfolder=$(select_subfolder)
echo "Selected subfolder: $selected_subfolder"

# Get the release type from the user
release_type=$(get_release_type)

# Change to the selected directory and release the crate
(
  cd "$selected_subfolder"
  cargo release "$release_type"
)

echo "Released ${selected_subfolder} with a $release_type version bump."

