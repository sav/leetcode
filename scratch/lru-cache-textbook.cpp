
#include <iostream>
#include <list>
#include <unordered_map>
#include <unordered_map>

#include <sstream>
#include <vector>
#include <string>

class LRUCache {
    int capacity;
    std::list<std::pair<int,int>> lst;
    std::unordered_map<int, std::list<std::pair<int,int>>::iterator> map;

public:
    LRUCache(int cap) : capacity(cap) {}

    int get(int key) {
        if (map.find(key) == map.end()) return -1;
        lst.splice(lst.begin(), lst, map[key]);
        return map[key]->second;
    }

    void put(int key, int value) {
        if (map.find(key) != map.end()) {
            map[key]->second = value;
            lst.splice(lst.begin(), lst, map[key]);
        } else {
            if ((int)map.size() == capacity) {
                map.erase(lst.back().first);
                lst.pop_back();
            }
            lst.push_front({key, value});
            map[key] = lst.begin();
        }
    }
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

