#include <algorithm>
#include <iostream>
#include <queue>
#include <unordered_map>
#include <unordered_set>
#include <utility>
#include <vector>

class Twitter {
    typedef int time_type;

  public:
    static constexpr int FEED_MAX = 10;

    void postTweet(int user, int tweet) {
        tweets[user].push_back({time++, tweet});
    }

    void follow(int user, int who) { follows[user].insert(who); }

    void unfollow(int user, int who) { follows[user].erase(who); }

    std::vector<int> getNewsFeed(int user) {
        std::priority_queue<std::pair<time_type, int>> pq;

        int n = tweets[user].size();
        for (int i = 0; i < n && i < FEED_MAX; ++i) {
            pq.push(tweets[user][n - i - 1]);
        }
        for (auto const& f : follows[user]) {
            int n = tweets[f].size();
            for (int i = 0; i < n && i < FEED_MAX; ++i) {
                pq.push(tweets[f][n - i - 1]);
            }
        }

        std::vector<int> recent;
        while (!pq.empty() && recent.size() < FEED_MAX) {
            recent.push_back(pq.top().second);
            pq.pop();
        }

        return recent;
    }

  private:
    time_type time = 0;
    std::unordered_map<int, std::vector<std::pair<time_type, int>>> tweets;
    std::unordered_map<int, std::unordered_set<int>> follows;
};

int main() {
    Twitter t;

    t.follow(0, 1);
    t.follow(0, 2);

    t.postTweet(1, 10);
    t.postTweet(2, 20);
    t.postTweet(1, 11);
    t.postTweet(2, 21);
    t.postTweet(1, 12);
    t.postTweet(2, 22);
    t.postTweet(1, 13);
    t.postTweet(2, 23);
    t.postTweet(1, 14);
    t.postTweet(2, 24);
    t.postTweet(1, 15);
    t.postTweet(2, 25);

    auto recent = t.getNewsFeed(0);
    for (auto tweet : recent) {
        std::cout << tweet << " ";
    }
    std::cout << std::endl;

    t.unfollow(0, 2);

    recent = t.getNewsFeed(0);
    for (auto tweet : recent) {
        std::cout << tweet << " ";
    }
    std::cout << std::endl;

    return 0;
}
