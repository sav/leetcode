#include <iostream>
#include <vector>
#include <utility>
#include <cassert>
#include <climits>

struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
    /// Returns the list node containing the minimum value among all the head nodes.
    std::pair<int, ListNode*> min(std::vector<ListNode*>& lists, int k) {
        std::pair<int, ListNode*> result = std::make_pair(-1, nullptr);
        for (int i = 0; i < k; ++i) {
            if (lists[i] == nullptr) continue;
            if (result.second == nullptr || lists[i]->val < result.second->val) {
                result.first = i;
                result.second = lists[i];
            }
        }
        return result;
    }

public:
    ListNode* mergeKLists(std::vector<ListNode*>& lists) {
        int k = lists.size();
        if (k == 0) return {};
        if (k == 1) return lists[0];

        auto [h, head] = min(lists, k);
        ListNode *result = head;

        while (h != -1) {
            lists[h] = head->next;
            auto next = min(lists, k);
            head->next = next.second;
            h = next.first, head = next.second;
        }
        return result;
    }
};

ListNode* make_list(std::initializer_list<int> vals) {
    ListNode dummy;
    ListNode* tail = &dummy;
    for (int v : vals) {
        tail->next = new ListNode(v);
        tail = tail->next;
    }
    return dummy.next;
}

std::vector<int> to_vec(ListNode* head) {
    std::vector<int> out;
    while (head) { out.push_back(head->val); head = head->next; }
    return out;
}

int main() {
    Solution sol;

    std::vector<ListNode*> lists = {
        make_list({-10, -3, 0, 5, 5, 100}),   // negatives, zero, duplicates
        nullptr,                              // empty list
        make_list({-5}),                      // single element
        make_list({5, 5, 5, 5}),              // all duplicates
        make_list({-10, 0, 100}),             // overlapping range with list 0
        nullptr,                              // another empty
        make_list({INT_MIN, 0, INT_MAX}),     // extreme values
    };

    ListNode* merged = sol.mergeKLists(lists);
    std::vector<int> result = to_vec(merged);

    // Verify sorted order
    for (int i = 1; i < result.size(); ++i)
        assert(result[i] >= result[i - 1]);

    // Verify element count: 6 + 0 + 1 + 4 + 3 + 0 + 3 = 17
    assert(result.size() == 17);

    for (int v : result) std::cout << v << " ";
    std::cout << "\n";
}
