#include <iostream>
#include <list>
#include <unordered_map>
#include <unordered_map>

#include <sstream>
#include <vector>
#include <string>

class LRUCache {
    public:
        LRUCache(int cap) : cap(cap) {}

        void update(int key) {
            used.splice(used.begin(), used, nodes[key]);
        }

        void evict() {
            auto lru = used.back();
            used.pop_back();
            nodes.erase(lru);
            data.erase(lru);
        }

        int get(int key) {
            if (data.find(key) == data.end()) {
                return -1;
            }
            update(key);
            return data[key];
        }

        void put(int key, int value) {
            bool contains = data.find(key) != data.end();
            if (!contains && data.size() >= cap) {
                evict();
            }
            if (!contains) {
                used.push_front(key);
                nodes[key] = used.begin();
            }
            data[key] = value;
            if (contains) {
                update(key);
            }
        }

        friend std::ostream& operator<<(std::ostream& os, LRUCache& cache) {
            os << "[debug] { ";
            for (auto [key, val]: cache.data) {
                os << "{" << key << ","  << val << "} ";
            }
            os << "} - ";

            os << "[ ";
            for (auto key: cache.used) {
                os << key << " ";
            }
            os << "]";
            return os;
        }

    private:
        int cap;
        std::unordered_map<int, int> data;
        std::list<int> used;
        std::unordered_map<int, std::list<int>::iterator> nodes;
};

int main() {
    std::string ops_line, args_line;
    std::getline(std::cin, ops_line);
    std::getline(std::cin, args_line);

    std::vector<std::string> ops;
    for (int i = 0; i < static_cast<int>(ops_line.size()); i++) {
        if (ops_line[i] == '"') {
            int j = ops_line.find('"', i + 1);
            ops.push_back(ops_line.substr(i + 1, j - i - 1));
            i = j;
        }
    }

    std::vector<std::vector<int>> args;
    for (int i = 0; i < static_cast<int>(args_line.size()); i++) {
        if (args_line[i] == '[' && (i + 1 >= (int)args_line.size() || args_line[i + 1] != '[')) {
            int j = args_line.find(']', i + 1);
            std::string inner = args_line.substr(i + 1, j - i - 1);
            std::vector<int> nums;
            if (!inner.empty()) {
                std::stringstream ss(inner);
                std::string tok;
                while (std::getline(ss, tok, ','))
                    nums.push_back(std::stoi(tok));
            }
            args.push_back(nums);
            i = j;
        }
    }

    LRUCache* cache = nullptr;
    for (int i = 0; i < static_cast<int>(ops.size()); i++) {
        if (ops[i] == "LRUCache") {
            cache = new LRUCache(args[i][0]);
        } else if (ops[i] == "put") {
            cache->put(args[i][0], args[i][1]);
        } else if (ops[i] == "get") {
            std::cout << cache->get(args[i][0]) << std::endl;
        }
    }
    delete cache;

    return 0;
}
