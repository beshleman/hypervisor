#!/bin/bash

aarch64-linux-gnu-gdb -ex "target remote localhost:1234" -ex "hb *0x40000650"
