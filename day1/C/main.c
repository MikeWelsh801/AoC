#include <inttypes.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int compare(const void *a, const void *b) { return *((int *)b) - *((int *)a); }

int main() {
  FILE *fptr = fopen("input.txt", "r");
  char line[1000];

  if (fptr == NULL)
    printf("Couldn't read file.");

  int count = 0;
  int cal_arr[500] = {0};
  size_t cal_arr_size = 0;
  int index = 0;

  while (fgets(line, 1000, fptr)) {
    if (strcmp(line, "\n") == 0) {
      cal_arr[index++] = count;
      cal_arr_size++;
      count = 0;
      continue;
    }

    uintmax_t num = strtoumax(line, NULL, 10);
    count += num;
  }

  qsort(cal_arr, cal_arr_size, sizeof(int), compare);
  printf("Answer 1: %d\n", cal_arr[0]);
  printf("Answer 2: %d\n", cal_arr[0] + cal_arr[1] + cal_arr[2]);

  fclose(fptr);
  return 0;
}
