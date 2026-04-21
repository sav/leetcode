#include <cassert>
#include <deque>
#include <functional>
#include <iostream>
#include <print>
#include <set>
#include <sstream>
#include <string>
#include <unordered_set>
#include <vector>

class Solution {
    struct node {
        int value;
        std::vector<int> next;
    };
    std::unordered_map<int, struct node> nodes;
    std::unordered_set<int> heads;
    std::unordered_map<int, int> degree;

    std::vector<int> walk(int numCourses) {
        std::deque<int> queue{heads.begin(), heads.end()};
        std::vector<int> result;
        while (!queue.empty()) {
            auto node = queue.front();
            queue.pop_front();
            result.push_back(node);
            for (int child : nodes[node].next) {
                degree[child]--;
                if (degree[child] == 0) {
                    queue.push_back(child);
                }
            }
        }

        if (result.size() != numCourses) return {};
        return result;
    }

  public:
    std::vector<int> findOrder(int numCourses,
                               std::vector<std::vector<int>> &prerequisites) {
        for (int course = 0; course < numCourses; ++course) {
            heads.insert(course);
        }
        for (std::vector<int> &prereq : prerequisites) {
            int to = prereq[0], need = prereq[1];
            nodes[to].value = to;
            nodes[need].value = need;
            nodes[need].next.push_back(to);
            degree[to]++;
            heads.erase(to);
        }
        return walk(numCourses);
    }
};

int main() {
    int n;
    char c;

    std::cin >> n;
    std::cin.ignore();
    std::string line;
    std::getline(std::cin, line);
    std::istringstream ss(line);
    std::vector<std::vector<int>> v;

    ss >> c; // [
    while (ss >> c && c == '[') {
        int x, y;
        ss >> x;
        ss >> c;
        assert(c == ',');
        ss >> y;
        v.push_back(std::vector<int>{x, y});
        ss >> c;
        assert(c == ']');
        ss >> c;
        if (c != ',')
            break;
    }

    auto result = Solution{}.findOrder(n, v);
    std::print("{{ ");
    for (int node : result) {
        std::print("{} ", node);
    }
    std::println("}}");
    return 0;
}
