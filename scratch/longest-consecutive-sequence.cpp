// clang++ -Wall -std=c++23 longest-consecutive-sequence.cpp

#include <algorithm>
#include <iostream>
#include <print>
#include <sstream>
#include <string>
#include <unordered_set>
#include <vector>

class Solution {
    int search(int n, std::unordered_set<int> &map,
               std::unordered_set<int> &visited) {
        int acc = 1, left = n, right = n;
        visited.insert(n);
        while (map.count(left - 1) > 0 || map.count(right + 1) > 0) {
            if (map.count(left - 1) > 0) {
                visited.insert(left - 1);
                left--, acc++;
            }
            if (map.count(right + 1) > 0) {
                visited.insert(right + 1);
                right++, acc++;
            }
        }
        return acc;
    }

  public:
    int longestConsecutive(std::vector<int> &nums) {
        std::unordered_set<int> map(nums.begin(), nums.end()), visited;
        int max = 0;
        for (auto &num : nums) {
            if (!visited.contains(num)) {
                int cur = search(num, map, visited);
                max = std::max(cur, max);
            }
        }
        return max;
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
