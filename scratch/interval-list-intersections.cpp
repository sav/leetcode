#include <algorithm>
#include <cassert>
#include <iostream>
#include <print>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

std::vector<int> parse_interval(std::basic_stringstream<char> &ss) {
    int first, second;
    char comma, close;
    ss >> first >> comma >> second >> close;
    return {first, second};
}

std::vector<std::vector<int>> parse_list_of_intervals(std::string const &line) {
    std::vector<std::vector<int>> result;
    std::stringstream ss(line);
    char ch;
    ss >> ch;
    assert(ch == '[');
    while (ss >> ch) {
        if (ch == '[')
            result.push_back(parse_interval(ss));
        else if (ch == ',')
            continue;
        else
            assert(ch == ']');
    }
    return result;
}

std::ostream &operator<<(std::ostream &os,
                         std::vector<std::vector<int>> const &list) {
    os << "[";
    for (std::size_t i = 0; i < list.size(); ++i) {
        os << "[" << list[i][0] << ", " << list[i][1] << "]";
        if (i < list.size() - 1)
            os << ", ";
    }
    os << "]";
    return os;
}

class Solution {
  public:
    bool intersects(std::vector<int> const &a, std::vector<int> const &b) {
        return a[0] <= b[0] ? a[1] >= b[0] : b[1] >= a[0];
    }

    std::vector<std::vector<int>>
    intervalIntersection(std::vector<std::vector<int>> &a,
                         std::vector<std::vector<int>> &b) {
        std::vector<std::vector<int>> result;
        int i = 0, j = 0;
        while (i < a.size() && j < b.size()) {
            if (intersects(a[i], b[j])) {
                result.push_back({
                    std::max(a[i][0], b[j][0]),
                    std::min(a[i][1], b[j][1]),
                });
            }
            if (a[i][1] < b[j][1]) {
                i++;
            } else {
                j++;
            }
        }
        return result;
    }
};

int main() {
    std::string line;

    std::getline(std::cin, line);
    auto list_1st = parse_list_of_intervals(line);

    std::getline(std::cin, line);
    auto list_2nd = parse_list_of_intervals(line);

    auto result = Solution().intervalIntersection(list_1st, list_2nd);
    std::cout << result << std::endl;

    return 0;
}
