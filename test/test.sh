# This relies on the first argument being a path to the implementation under test
TEST_BIN=$1
if [[ -z "$TEST_BIN" ]]; then
    echo "ERROR: Implementation under test not set!"
    exit 1
fi
if [[ ! -f "$TEST_BIN" ]]; then
    echo "ERROR: '$TEST_BIN' does not exist or is not a file"
    exit 1
fi

# Cleanup from previously
rm -rf fake_home

mkdir fake_home
mkdir fake_home/dir_a
mkdir fake_home/dir_a/sub_a1
mkdir fake_home/dir_a/sub_a2
mkdir fake_home/dir_b
mkdir fake_home/dir_b/su
mkdir fake_home/.config
cat > fake_home/.config/to <<- EOF
{
    "directories": [
        "$(readlink -e fake_home/dir_a)",
        "$(readlink -e fake_home/dir_b)"
    ]
}
EOF

function check() {
    NAME=$1
    EXP=$2
    ACT=$3

    RESTORE='\033[0m'
    RED='\033[00;31m'
    GREEN='\033[00;32m'
    if [[ "$EXP" == "$ACT" ]]; then
        echo -e "[${GREEN}PASS${RESTORE}] $NAME"
    else
        echo -e "[${RED}FAIL${RESTORE}] $NAME"
        echo "Expected: '$EXP'"
        echo "Actual:   '$ACT'"
    fi

}

EXP=$'sub_a1\nsub_a2'
ACT="$(HOME=$(readlink -e fake_home) $TEST_BIN list sub)"
check "Lists the correct subdirectories"  "$EXP"  "$ACT"

EXP="sub_a2"
ACT="$(HOME=$(readlink -e fake_home) $TEST_BIN list sub_a2)"
check "Distinguishes based on directory name"  "$EXP"  "$ACT"

EXP=$'su\nsub_a1\nsub_a2'
ACT="$(HOME=$(readlink -e fake_home) $TEST_BIN list su)"
check "Lists directories from multiple base dirs"  "$EXP"  "$ACT"

EXP=$(readlink -e fake_home/dir_b/su)
ACT="$(HOME=$(readlink -e fake_home) $TEST_BIN go su)"
check "command 'go' returns a single result if multiple match"  "$EXP"  "$ACT"


rm -rf fake_home
