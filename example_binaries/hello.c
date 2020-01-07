#include <stdio.h>
#include <stdlib.h>

int main() {
  puts("Hello world!");
  char name[64];
  puts("What's your name?");
  fgets((char*) &name, 64, stdin);
  printf("Hello, %s!", name);
}
