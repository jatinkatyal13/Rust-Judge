#!/usr/bin/env bash

WORKDIR=$1

g++ -o $WORKDIR/exe $WORKDIR/source.cpp 2> $WORKDIR/stderr 1> $WORKDIR/stdout
if test -f "$WORKDIR/exe"
then
  chmod 777 $WORKDIR/exe
  $WORKDIR/exe < $WORKDIR/stdin 1> $WORKDIR/stdout 2> $WORKDIR/stderr
fi
