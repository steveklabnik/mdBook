#!/bin/bash

red=$'\e[1;31m'
grn=$'\e[1;32m'
yel=$'\e[0;33m'
blu=$'\e[1;34m'
mag=$'\e[1;35m'
cyn=$'\e[1;36m'
white=$'\e[0m'


#--------------------------
function help() {

echo "Usage:"
echo ""


echo "  $0"
awk -F ')' '/\|--.*shift/ && ! /awk/ { print $1} ' $0

cat <<HELP

This shell script takes the path to two mdbook binaries; 
and a third parameter of the book directory where one or more book.toml's live.

It will then build every "book.toml" book in the directory, twice, with the two mdbook versions. 
once complete, it will compare the two directories. 

Comparison is difficult, but the following applies

OK   - if the two output directories are identical
WARN - status==OK, if the two directories are not identical, but the size of the second is +/- 5% of the first.
FAIL - if the size of the second is greater than +/- 5% of the first.

HELP

}
#--------------------------

if [[ $# -eq 0 ]]; then
  $0 --help
  exit 1
fi

if [[ $# -ne 10 && $# -ne 1 ]]; then
  $0 --help
  exit 1
fi


while [[ $# -ge 1 ]]; do
    case "$1" in
        -v1|--version1)    VERSION1="${2}";     shift 2 ;;
        -v2|--version2)    VERSION2="${2}";     shift 2 ;;
        -p|--book-path)    BOOK_PATH="${2}";    shift 2 ;;
        -n|--book-name)    BOOK_NAME="${2}";    shift 2 ;;
        -d|--working-dir)  WORKING_DIR="${2}";  shift 2 ;;
        -h|--help)         help            ;  exit 0 ;;
        *) echo "Unknown arg [$1]. Error parsing arguments" ; exit 1 ;;
    esac
done

function build_book_and_check() {
  binary=$1
  book_toml=$2
  output_dir=$3

  dir=$(dirname $book_toml)
  mkdir -p ${output_dir} 2> /dev/null

  echo "$blu:: Building $BOOK_NAME with $binary${white}"

  (cd $dir && $binary build --dest-dir ${output_dir} )

  retVal=$?
  if [ $retVal -ne 0 ]; then
     echo "$red:: Failed to build $book_toml with $binary"
  fi
  return

}

#--------------------------

books=$(find ${BOOK_PATH} -type f -name book.toml)
basename_1=$(basename $VERSION1)
basename_2=$(basename $VERSION2)

echo :: $VERSION1
echo :: $VERSION2

for book_toml in $books
do

   build_book_and_check $VERSION1 $book_toml ${WORKING_DIR}/${basename_1}
   build_book_and_check $VERSION2 $book_toml ${WORKING_DIR}/${basename_2}

   DIFF=$(diff -rq ${WORKING_DIR}/${basename_1} ${WORKING_DIR}/${basename_2} ) 
   if [ "$DIFF" != "" ] 
   then
      echo "$red:: $book_toml the directories differ$white"
      echo $mag  - ${WORKING_DIR}/${basename_1}$white
      echo $mag  - ${WORKING_DIR}/${basename_2}$white

      dir_1_size=$(du -s ${WORKING_DIR}/${basename_1} | awk '{print $1*512}' )
      dir_2_size=$(du -s ${WORKING_DIR}/${basename_2} | awk '{print $1*512}' )

     if (( dir_2_size / dir_1_size * 100 > 105 || dir_2_size / dir_1_size * 100 < 95 )); then
        echo $red:: FAIL outside Tolerance +/- 5% $white
        FAIL=1
     else
        echo $yel:: WARN differed, but inside tolerance +/- 5% $white
     fi
   else 
      echo $grn:: OK $book_toml is identical$white
      echo $cyn  - ${WORKING_DIR}/${basename_1}$white
      echo $cyn  - ${WORKING_DIR}/${basename_2}$white
   fi
done

if [[ $FAIL -eq 1 ]];then
  exit 1
fi
