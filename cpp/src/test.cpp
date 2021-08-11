#include <iostream>

template <typename... T> struct ItemArray {};

namespace details {
template <typename T> constexpr inline bool is_item_array_help_v = false;

template <typename... T>
constexpr inline bool is_item_array_help_v<ItemArray<T...>> = true;
} // namespace details

template <typename T>
constexpr inline bool is_item_array_v =
    details::is_item_array_help_v<std::decay_t<T>>;

int main() {
  std::cout << is_item_array_v<bool> << "\n";
  std::cout << is_item_array_v<ItemArray<int>> << "\n";
  std::cout << is_item_array_v<const ItemArray<int>> << "\n";
  std::cout << is_item_array_v<ItemArray<int> &> << "\n";
  std::cout << is_item_array_v<const ItemArray<int> &> << "\n";
}