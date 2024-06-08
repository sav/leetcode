#include <cctype> // for std::isdigit
#include <iostream>
#include <map>
#include <sstream>
#include <string>

class Solution {
  public:
    bool isHappy(int n) {
        std::map<int, bool> map;

        std::string s = std::to_string(n);
        int sum = 0;

        while (sum != 1) {
            sum = 0;

            for (char c : s) {
                int x = c - '0';
                sum += x * x;
            }

            if (map.find(sum) != map.end()) {
                return false;
            }

            map[sum] = true;
            s = std::to_string(sum);
        }

        return true;
    }
};

int main() {
    Solution sol;

    bool result = sol.isHappy(1);
    std::cout << " n=1, result=" << (result ? "true" : "false") << std::endl;

    result = sol.isHappy(19);
    std::cout << "n=10, result=" << (result ? "true" : "false") << std::endl;

    result = sol.isHappy(2);
    std::cout << " n=2, result=" << (result ? "true" : "false") << std::endl;

    return 0;
}
