#include <chrono>
#include <future>
#include <iostream>
#include <numeric>
#include <vector>
using namespace std::chrono_literals;

auto add(std::vector<std::int32_t> &numbers) { numbers.push_back(42); }

auto sum(std::vector<std::int32_t> &numbers) -> std::int32_t {
  auto begin = std::begin(numbers);
  auto end = std::end(numbers);
  std::this_thread::sleep_for(200ms);
  return std::reduce(begin, end, 0, std::plus<>{});
}

int main() {
  std::vector<std::int32_t> numbers(100,1);
  auto sum_future = std::async(std::launch::async, sum, std::ref(numbers));
  std::this_thread::sleep_for(100ms);
  add(numbers);
  std::cout << "The sum is " << sum_future.get() << "\n";
}