#ifndef LEETCODE_H_
#define LEETCODE_H_

#include <ostream>
#include <algorithm>
#include <iterator>

namespace leetcode {

    template <typename T>
    void print_vector(std::ostream& os, std::vector<T> vec) {
        os << "[ ";
        std::copy(vec.begin(), vec.end(), std::ostream_iterator<T>(os, " "));
        os << "]" << std::endl;
    }

}
    
#endif // LEETCODE_H_
