#include <unordered_map>
#include <list>

class LRUCache {
public:
    LRUCache(int capacity) {
        this->capacity_ = capacity;
    }
    
    int get(int key) {
        auto it = this->cache_.find(key);
        if (it == this->cache_.end()) {
            return -1;
        }
        auto val = it->second;
        this->queue_.remove(it);
        this->queue_.push_back(it);
        return val;
    }
    
    void put(int key, int value) {
        if (this->cache_.size() >= this->capacity_) {
            auto lru = this->queue_.front();
            this->queue_.pop_front();
            this->cache_.erase(lru);
        }
        auto it = this->cache_.insert({key, value});
        this->queue_.push_back(it.first);
    }

    std::size_t capacity_;
    std::list<std::unordered_map<int, int>::iterator> queue_;
    std::unordered_map<int, int> cache_;
};

