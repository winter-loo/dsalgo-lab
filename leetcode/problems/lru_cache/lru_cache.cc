// cmake -B build -DCMAKE_BUILD_TYPE=Debug -DCMAKE_EXPORT_COMPILE_COMMANDS=ON
#include <iostream>
#include <list>
#include <unordered_map>
#include <utility>
#include <vector>

class LRUCache {
public:
  LRUCache(int capacity) { this->capacity_ = capacity; }

  int get(int key) {
    auto it = this->cache_.find(key);
    if (it == this->cache_.end()) {
      return -1;
    }
    // Save a copy of the pair before erasing
    std::pair<int, int> pair_copy = *(it->second);
    int val = pair_copy.second;

    // Erase from current position
    this->list_.erase(it->second);

    // Insert the copy at the end and update the cache
    auto new_it = this->list_.insert(this->list_.end(), pair_copy);
    it->second = new_it;

    return val;
  }

  void put(int key, int value) {
    auto it = this->cache_.find(key);
    if (it != this->cache_.end()) {
      // Key exists, update value
      // Erase from current position first
      this->list_.erase(it->second);

      // Insert new pair at the end and update cache
      auto new_it = this->list_.insert(this->list_.end(),
                                       std::pair<int, int>(key, value));
      it->second = new_it;
    } else {
      // Key doesn't exist
      if (this->cache_.size() >= this->capacity_) {
        // Remove least recently used item (front of list)
        auto lru = this->list_.front();
        auto lru_it = this->cache_.find(lru.first);
        if (lru_it != this->cache_.end()) {
          this->cache_.erase(lru_it);
        }
        this->list_.pop_front();
      }

      // Add new key-value pair
      auto list_it = this->list_.insert(this->list_.end(),
                                        std::pair<int, int>(key, value));
      this->cache_.insert({key, list_it});
    }
  }

  std::size_t capacity_;
  std::list<std::pair<int, int>> list_;
  std::unordered_map<int, std::list<std::pair<int, int>>::iterator> cache_;
};

int main() {
  const std::vector<std::string> operations = {
      "LRUCache", "put", "put", "put", "put", "put", "get", "put", "get",
      "get",      "put", "get", "put", "put", "put", "get", "put", "get",
      "get",      "get", "get", "put", "put", "get", "get", "get", "put",
      "put",      "get", "put", "get", "put", "get", "get", "get", "put",
      "put",      "put", "get", "put", "get", "get", "put", "put", "get",
      "put",      "put", "put", "put", "get", "put", "put", "get", "put",
      "put",      "get", "put", "put", "put", "put", "put", "get", "put",
      "put",      "get", "put", "get", "get", "get", "put", "get", "get",
      "put",      "put", "put", "put", "get", "put", "put", "put", "put",
      "get",      "get", "get", "put", "put", "put", "get", "put", "put",
      "put",      "get", "put", "put", "put", "get", "get", "get", "put",
      "put",      "put", "put", "get", "put", "put", "put", "put", "put",
      "put",      "put"};
  std::vector<std::vector<int>> operands = {
      {10},     {10, 13}, {3, 17},  {6, 11},  {10, 5}, {9, 10},  {13},
      {2, 19},  {2},      {3},      {5, 25},  {8},     {9, 22},  {5, 5},
      {1, 30},  {11},     {9, 12},  {7},      {5},     {8},      {9},
      {4, 30},  {9, 3},   {9},      {10},     {10},    {6, 14},  {3, 1},
      {3},      {10, 11}, {8},      {2, 14},  {1},     {5},      {4},
      {11, 4},  {12, 24}, {5, 18},  {13},     {7, 23}, {8},      {12},
      {3, 27},  {2, 12},  {5},      {2, 9},   {13, 4}, {8, 18},  {1, 7},
      {6},      {9, 29},  {8, 21},  {5},      {6, 30}, {1, 12},  {10},
      {4, 15},  {7, 22},  {11, 26}, {8, 17},  {9, 29}, {5},      {3, 4},
      {11, 30}, {12},     {4, 29},  {3},      {9},     {6},      {3, 4},
      {1},      {10},     {3, 29},  {10, 28}, {1, 20}, {11, 13}, {3},
      {3, 12},  {3, 8},   {10, 9},  {3, 26},  {8},     {7},      {5},
      {13, 17}, {2, 27},  {11, 15}, {12},     {9, 19}, {2, 15},  {3, 16},
      {1},      {12, 17}, {9, 1},   {6, 19},  {4},     {5},      {5},
      {8, 1},   {11, 7},  {5, 2},   {9, 28},  {1},     {2, 2},   {7, 4},
      {4, 22},  {7, 24},  {9, 26},  {13, 28}, {11, 26}};

  // Store results
  std::vector<int> results;
  LRUCache *cache = nullptr;

  // Process all operations
  const int numOperations = operations.size();

  for (int i = 0; i < numOperations; i++) {
    std::string op = operations[i];

    if (op == "LRUCache") {
      // Initialize the cache with capacity
      int capacity = operands[i][0];
      cache = new LRUCache(capacity);
      results.push_back(-1); // null result for constructor
    } else if (op == "put") {
      // Put key-value pair
      int key = operands[i][0];
      int value = operands[i][1];
      cache->put(key, value);
      results.push_back(-1); // null result for put operation
    } else if (op == "get") {
      // Get value by key
      int key = operands[i][0];
      int result = cache->get(key);
      results.push_back(result);
    }
  }

  // Print results
  std::cout << "Results: [";
  for (size_t i = 0; i < results.size(); i++) {
    if (results[i] == -1 &&
        (operations[i] == "LRUCache" || operations[i] == "put")) {
      std::cout << "null";
    } else {
      std::cout << results[i];
    }

    if (i < results.size() - 1) {
      std::cout << ", ";
    }
  }
  std::cout << "]" << std::endl;

  // Clean up
  delete cache;
  return 0;
}
