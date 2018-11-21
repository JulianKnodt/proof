#include <stdio.h>

#define bool_f 0x2f
#define bool_t 0x6f
#define nil 63
#define fixnum_mask 0x03
#define fixnum_tag 0x00
#define fixnum_shift 2
#define char_mask 15
#define char_shift 8

/*
Intended to be compiled with an assembly file created by the compiler

To link, run `gcc runtime.c <FILENAME.s>
*/

int scheme();

static void print_res(unsigned int x) {
  if((x & fixnum_mask) == fixnum_tag) {
    printf("%d", (int)x >> fixnum_shift);
  } else if (x == bool_f) {
    printf("#f");
  } else if (x == bool_t) {
    printf("#t");
  } else if (x == nil) {
    printf("nil");
  }else if ((x & char_mask) == char_mask) {
    printf("#\\%c", x >> char_shift);
  }
  printf("\n");
}

int main() {
  print_res(scheme());
  return 0;
}
