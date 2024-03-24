#!/bin/bash

# Usage: ./IOtester.sh inputdir_apl inputdir_py expected_dir output_dir

# Check if the number of arguments provided is not equal to 2
if [ "$#" -ne 4 ]; then
    echo "Usage: sh $0 inputdir_apl inputdir_py expected_dir output_dir"
    exit 1
fi

# Check if the first argument is not a directory or doesn't exist
if [ ! -d "$1" ]; then
    echo "Error: '$1' is not a directory or does not exist."
    exit 1
fi

# Check if the second argument is not a directory or doesn't exist
if [ ! -d "$2" ]; then
    echo "Error: '$2' is not a directory or does not exist."
    exit 1
fi

# Check if the third argument is not a directory or doesn't exist
if [ ! -d "$3" ]; then
    echo "Error: '$3' is not a directory or does not exist."
    exit 1
fi


# Check if the third argument is not a directory or doesn't exist
if [ ! -d "$3" ]; then
    echo "Error: '$3' is not a directory or does not exist."
    exit 1
fi

# Assign the first command-line argument to a variable
inputdir_apl="$1"

# Assign the second command-line argument to another variable
inputdir_py="$2"
# Assign the second command-line argument to another variable
expected_dir="$3"
# Assign the second command-line argument to another variable
output_dir="$4"

# Check if the directory is empty
if [ -z "$(find "${output_dir}" -maxdepth 1 -type f)" ]; then
    echo "Directory '${output_dir}' is empty."
    else 
        # clear the output directory
        for f0 in "${output_dir}"*.output
        do
            # echo $f0
            if [ -f "$f0" ]; then
                rm "$f0"
                echo "Removed file: $f0"
            else
                echo "Tried to remove a file without the .output extension: $f0"
                exit 1
            fi
        done
fi

# change to .apl or just run for all files
# save the input name to later use for outputdir / check against expected dir
for f1 in "${inputdir_apl}"*.apl
do
    # run all the .apl filesed" > ${f}.actual
    f_no_path="${f1##*/}"   # Remove path
    f_no_ext="${f_no_path%.*}"  # Remove extension

    # apl -f "$inputdir_apl${f_no_ext}.apl" --noCIN --OFF --safe -s -q  > "$3${f_no_ext}.output"

    # TODO add python or c++ and redirect to output , change apl to be expected 
    apl -f "${inputdir_apl}${f_no_ext}.apl" --noCIN --OFF --safe -s -q  > "${expected_dir}${f_no_ext}.expected"
    
    

    python3 "${inputdir_py}${f_no_ext}.py" > "${output_dir}${f_no_ext}.output"
    # echo $f
    # use expected and actual when comparing apl output to py or cuda later
    if ! diff -u "${expected_dir}${f_no_ext}.expected" "${output_dir}${f_no_ext}.output"
    then
        echo ">>> Failed :-("
        exitcode=1
    else
        echo ">>> Success :-)"
    fi
done
exit $exitcode