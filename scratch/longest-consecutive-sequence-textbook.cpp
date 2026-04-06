// clang++ -Wall -std=c++23 longest-consecutive-sequence.cpp

#include <algorithm>
#include <iostream>
#include <print>
#include <sstream>
#include <string>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
  public:
    int longestConsecutive(vector<int> &nums) {
        unordered_set<int> s(nums.begin(), nums.end());
        int best = 0;
        for (int n : s) {
            if (!s.count(n - 1)) { // n is a sequence start
                int len = 1;
                while (s.count(n + len))
                    len++;
                best = max(best, len);
            }
        }
        return best;
    }
};

template <typename T> std::vector<T> read_vector() {
    std::string line;
    std::getline(std::cin, line);
    std::istringstream ss(line);
    std::vector<T> vec;
    T n;
    while (ss >> n)
        vec.push_back(n);
    return vec;
}

int main() {
    auto v = read_vector<int>();
    auto res = Solution().longestConsecutive(v);
    std::cout << res << std::endl;
    return 0;
}
