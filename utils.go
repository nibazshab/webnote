package main

import (
    "math/rand"
    "time"
    "unsafe"
)

const letter_bytes = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
const (
    letter_idx_bits = 6
    letter_idx_mask = 1<<letter_idx_bits - 1
    letter_idx_max  = 63 / letter_idx_bits
)

var src = rand.NewSource(time.Now().UnixNano())

func rand_string() string {
    b := make([]byte, 4)
    for i, cache, remain := 3, src.Int63(), letter_idx_max; i >= 0; {
        if remain == 0 {
            cache, remain = src.Int63(), letter_idx_max
        }
        if idx := int(cache & letter_idx_mask); idx < len(letter_bytes) {
            b[i] = letter_bytes[idx]
            i--
        }
        cache >>= letter_idx_bits
        remain--
    }
    return *(*string)(unsafe.Pointer(&b))
}
