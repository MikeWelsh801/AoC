#include <algorithm>
#include <fstream>
#include <functional>
#include <iostream>
#include <stdio.h>
#include <string>
#include <vector>

using std::vector;

int main() {
  vector<int> elfCals;
  std::ifstream file;
  file.open("../elfCals.txt");

  int sum{0};
  std::string line;
  while (std::getline(file, line)) {
    if (line.empty()) {
      elfCals.push_back(sum);
      sum = 0;
    } else {
      sum += std::stoi(line);
    }
  }

  std::sort(elfCals.begin(), elfCals.end(), std::greater<int>());
  printf("Answer1: %i\n", elfCals[0]);
  printf("Answer2: %i\n", elfCals[0] + elfCals[1] + elfCals[2]);

  file.close();
}
