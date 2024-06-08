#include <algorithm>
#include <iostream>
#include <vector>

class Solution {
  public:
    bool containsNearbyDuplicate(std::vector<int>& nums, int k) {
        std::size_t n = std::size(nums);

        for (auto i = 0; i < n - 1; ++i) {
            for (auto j = i + 1; j < n && j <= k + i; ++j) {
                if (nums[i] == nums[j] && j - i <= k) {
                    return true;
                }
            }
        }

        return false;
    }
};

int main() {
    Solution sol;

    std::vector<int> in01 = {1, 2, 3, 1};
    bool res01 = sol.containsNearbyDuplicate(in01, 3);
    std::cout << (res01 ? "true" : "false") << std::endl;

    std::vector<int> in02 = {1, 0, 1, 1};
    bool res02 = sol.containsNearbyDuplicate(in02, 1);
    std::cout << (res02 ? "true" : "false") << std::endl;

    std::vector<int> in03 = {1, 2, 3, 1, 2, 3};
    bool res03 = sol.containsNearbyDuplicate(in03, 2);
    std::cout << (res03 ? "true" : "false") << std::endl;

    return 0;
}
