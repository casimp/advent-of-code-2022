LINE="========================"
AOC_TOKEN='session=53616c7465645f5f4ba233812318b689a16719a6ae1dfa0c61ab8162bf36bae762af4c6ab53f49b61f07b4e9814757119cf6a0e8ee00b10b08f736b7649a814c'
YEAR=2023
###################################
echo ${LINE}
echo "AOC ${YEAR} Day Templator"
echo ${LINE}
read -p "Enter day: " DAY
PADDED_DAY=`printf %02d $DAY`
FOLDER="day${PADDED_DAY}"
echo "Great! Starting by initializing the day folder...\n"
cargo new ${FOLDER}
echo ${LINE}
echo "Now getting the puzzle input...\n"
SAVE_PATH="${FOLDER}/src/input.txt"
PUZZLE_URL=https://adventofcode.com/${YEAR}/day/${DAY}/input
echo "Downloading: ${PUZZLE_URL}"
echo "Save path: ${SAVE_PATH}\n"
curl "${PUZZLE_URL}" -H "cookie: ${AOC_TOKEN}" -o ${SAVE_PATH}
echo ${LINE}
