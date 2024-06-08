#include <algorithm>
#include <iostream>
#include <vector>

#include "leetcode.hpp"

class Solution {
  public:
    std::vector<int> twoSum(std::vector<int>& nums, int target) {
        std::size_t n = nums.size();

        for (auto i = 0; i < n - 1; ++i) {
            for (auto j = i + 1; j < n; ++j) {
                int sum = nums[i] + nums[j];
                if (target == sum) {
                    return {i, j};
                }
            }
        }

        return {};
    }
};

int main() {
    Solution sol;

    std::vector<int> in01 = {2, 7, 11, 15};
    std::vector<int> res01 = sol.twoSum(in01, 9);
    leetcode::print_vector(std::cout, res01);

    std::vector<int> in02 = {3, 2, 4};
    std::vector<int> res02 = sol.twoSum(in02, 6);
    leetcode::print_vector(std::cout, res02);

    std::vector<int> in03 = {3, 3};
    std::vector<int> res03 = sol.twoSum(in03, 6);
    leetcode::print_vector(std::cout, res03);

    return 0;
}
