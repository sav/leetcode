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
#include <queue>

class Solution {
  public:
    std::vector<int> findOrder(int numCourses,
                               std::vector<std::vector<int>> &prerequisites) {
        std::vector<std::vector<int>> adj(numCourses);
        std::vector<int> indegree(numCourses, 0);

        for (auto &p : prerequisites) {
            adj[p[1]].push_back(p[0]);
            indegree[p[0]]++;
        }

        std::queue<int> q;
        for (int i = 0; i < numCourses; i++)
            if (indegree[i] == 0)
                q.push(i);

        std::vector<int> result;
        while (!q.empty()) {
            int node = q.front();
            q.pop();
            result.push_back(node);
            for (int child : adj[node]) {
                if (--indegree[child] == 0)
                    q.push(child);
            }
        }

        return result.size() == numCourses ? result : std::vector<int>{};
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
