#!/usr/bin/env sh
#
# Verifies multi-feature compilation, test runs, and documentation build.

set -e # stops on error

MSRV="1.70.0"
RCMD="rustup -v run $MSRV"

rustup override set $MSRV
rustup target add thumbv7m-none-eabi

# FAST CHECKS
cmd="$RCMD cargo cF"; echo "Fast: std, safe\n$ " $cmd; $cmd
cmd="$RCMD cargo craF"; echo "Fast: std, safe, all, release\n$" $cmd; $cmd
cmd="$RCMD cargo cauF"; echo "Fast: std, unsafe, all \n$" $cmd; $cmd
cmd="$RCMD cargo cNauF"; echo "Fast: no-std, no-alloc, unsafe, all\n$" $cmd; $cmd

# cmd="cargo tF"; echo $cmd; $cmd # std, safe
# cmd="cargo +nightly ndF"; echo $cmd; $cmd

# FULL CHECKS
cmd="$RCMD cargo ca"; echo "Full: std, safe\n$" $cmd; $cmd
cmd="$RCMD cargo cNa"; echo "Full: std, safe\n$" $cmd; $cmd

cmd="$RCMD cargo t"; echo "Full: tests\n$" $cmd; $cmd

cmd="cargo +nightly nd"; echo "Full: docs\n$" $cmd; $cmd
