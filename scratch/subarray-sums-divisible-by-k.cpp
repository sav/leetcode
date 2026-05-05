/**
 * Given an integer array nums and an integer k, return the number of non-empty
 * subarrays that have a sum divisible by k. A subarray is a contiguous part of
 * an array.
 *
 * The solution uses prefix sum and modulo:
 *
 *   prefix[i] = a_0 + a_1 + ... + a_i
 *
 * A subarray from l to r has sum:
 *
 *  prefix[r] - prefix[l - 1]
 *
 * You want:
 *
 *  (prefix[r] - prefix[l-1]) % k = 0
 *
 * That's equivalent to:
 *
 *  prefix[r] % k = prefix[l-1] % k
 *
 * So the solution is: count how many pairs of prefix sums have the same
 * remainder modulo k.
 *
 * Why this works? If two prefix sums leave the same remainder when divided by
 * k, their difference is divisible by k. That difference corresponds exactly to
 * a subarray.
 */

#include <iostream>
#include <unordered_map>
#include <vector>

class Solution {
  public:
    int subarraysDivByK(std::vector<int> &nums, int k) {
        int acc = 0, prefix = 0;
        std::unordered_map<int, int> mods = {{0, 1}};
        for (int num : nums) {
            prefix += num;
            int mod = ((prefix % k) + k) % k; // handle negative numbers
            acc += mods[mod]++;
        }
        return acc;
    }
};

int main() {
    Solution sol;

    std::vector v1 = {2, 7, 6, 1, 4, 5};
    std::cout << sol.subarraysDivByK(v1, 3) << std::endl;

    std::vector v2 = {4, 5, 0, -2, -3, 1};
    std::cout << sol.subarraysDivByK(v2, 5) << std::endl;

    std::vector v3 = {5};
    std::cout << sol.subarraysDivByK(v3, 9) << std::endl;

    return 0;
}
